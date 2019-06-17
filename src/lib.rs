//! A platform agnostic driver to interface with the MAX31855
//! //!
//! //! This driver was built using [`embedded-hal`] traits.
//! //!
//! //! [`embedded-hal`]: https://docs.rs/embedded-hal/~0.1
//! //!

#![deny(missing_docs)]
#![deny(warnings)]
#![no_std]

extern crate embedded_hal as hal;
extern crate bit_field;

use hal::blocking::spi::Transfer;
use hal::spi::{Mode, Phase, Polarity};
use hal::digital::OutputPin;
use bit_field::BitField;

/// SPI Mode Configuration
pub const MODE: Mode = Mode {
    phase: Phase::CaptureOnFirstTransition,
    polarity: Polarity::IdleLow,
};

/// MAX31855 Driver
pub struct Max31855<SPI, CS> {
    spi: SPI,
    cs: CS,
}

/// MAX31855 specific errors
#[derive(Debug)]
pub struct Max31855Error {
    /// Thermocouple is sort circuited to Vcc
    scvfault: bool,
    /// Thermocouple is sort circuited to ground
    scgfault: bool,
    /// Thermocouple is open circuited
    ocfault: bool
}

/// Errors this drive can return
#[derive(Debug)]
pub enum Error<E> {
    /// Error caused by Max31855 sensor
    SensorError(Max31855Error),
    /// Error caused by SPI interface
    SPIError(E)
}

impl<SPI, CS, E> Max31855<SPI, CS>
    where SPI: Transfer<u8, Error = E>,
          CS:  OutputPin
{
    /// Creates a new driver from a SPI peripheral and a CS pin
    pub fn new(spi: SPI, cs: CS) -> Result<Self, E> {
        let max31855 = Max31855 { spi: spi, cs: cs };
        Ok(max31855)
    }

    /// Read data from SPI peripheral
    fn read_spi(&mut self) -> Result<Raw, E> {

        self.cs.set_low();

        let mut buffer = [0u8; 4];
        self.spi.transfer(&mut buffer)?;

        self.cs.set_high();

        // Combine array of u8 to a u32 (MSB to LSB)
        let r: u32 = ((buffer[0] as u32) << 24) |
                     ((buffer[1] as u32) << 16) |
                     ((buffer[2] as u32) <<  8) |
                      (buffer[3] as u32);

        Ok(
            Raw {
                temperature: self.to_i16(r.get_bits(18..32) as u16, SensorType::HotRefJunction),
                cold_reference: self.to_i16(r.get_bits(4..15) as u16, SensorType::ColdRefJunction),
                fault: r.get_bit(16) as bool,
                scv: r.get_bit(2) as bool,
                scg: r.get_bit(1) as bool,
                oc: r.get_bit(0) as bool,
            }
        )
    }

    /// Return the thermocouple temperature measurement
    pub fn read_thermocouple(&mut self, unit: Units) -> Result<f32, Error<E>> {

        let raw = match self.read_spi() {
            Ok(t) => t,
            Err(e) => return Err(Error::SPIError(e))
        };

        if raw.fault {
            Err(Error::SensorError(Max31855Error {
                        scvfault: raw.scv,
                        scgfault: raw.scg,
                        ocfault: raw.oc
                    }))

        } else {
            Ok(self.calibrate_thermocouple(raw.temperature, &unit))
        }
    }

    /// Returns all measurements from the MAX31855
    pub fn read_all(&mut self, unit: Units) -> Result<Measurement, E> {

        let raw = self.read_spi()?;

        Ok(
            Measurement {
                temperature: self.calibrate_thermocouple(raw.temperature, &unit),
                cold_reference: self.calibrate_reference(raw.cold_reference, &unit),
                fault: raw.fault,
                scv: raw.scv,
                scg: raw.scg,
                oc: raw.oc,
            }
        )
    }

    /// Interface to convert temperature measurements from u16 to i16. Supports two sensors
    ///     HotRefJunction which is the 14 bit measurement and ColdRefJunction which is the
    ///     12 bit measurement
    fn to_i16(&mut self, unsigned_val: u16, sensor_type: SensorType) -> i16 {
        match sensor_type {
            SensorType::HotRefJunction =>
                 self.convert(
		              unsigned_val,
		              Convert {bit_num: 13, divisor: 4, bit_shift: 2}
		         ),
            SensorType::ColdRefJunction =>
		         self.convert(
		              unsigned_val,
		              Convert {bit_num: 11, divisor: 16, bit_shift: 4}
		         )
        }
    }

    /// Converts a u16 to i16 with the Convert type structure
    fn convert(&mut self, unsigned_val: u16, c: Convert) -> i16 {
        if unsigned_val.get_bit(c.bit_num) as bool {
            ((unsigned_val << c.bit_shift) as i16) / c.divisor
        } else {
            unsigned_val as i16
        }
    }

    /// Calibrates the thermocouple (14 bit measurement)
    fn calibrate_thermocouple(&mut self, count: i16, unit: &Units) -> f32 {
        match *unit {
            Units::Count      => (count as f32), // for debugging
            Units::Celsius    => (count as f32) * 0.25,
            Units::Fahrenheit => (count as f32) * 0.45 + 32.0,
            Units::Kelvin     => (count as f32) * 0.25 + 273.15,
        }
    }

    /// Calibrates the cold reference junction (12 bit measurement)
    fn calibrate_reference(&mut self, count: i16, unit: &Units) -> f32 {
        match *unit {
            Units::Count      => (count as f32), // for debugging
            Units::Celsius    => (count as f32) * 0.0625,
            Units::Fahrenheit => (count as f32) * 0.1125 + 32.0,
            Units::Kelvin     => (count as f32) * 0.0625 + 273.15,
        }
    }
}

/// Units Enumeration
pub enum Units {
    /// Represent temperatures in ADC Counts
    Count,
    /// Represent temperatures in Celsius
    Celsius,
    /// Represent temperatures in Fahrenheit
    Fahrenheit,
    /// Represent temperatures in Kelvin
    Kelvin,
}

/// Sensor Types Enumeration
enum SensorType {
    HotRefJunction,
    ColdRefJunction,
}

/// Structure to convert different bit length
///   measurements from i16 to u16
struct Convert {
    bit_num: usize,
    divisor: i16,
    bit_shift: u8,
}

/// Calibrated measurements from MAX31855
#[allow(dead_code)]
pub struct Measurement {
    /// Thermocouple temperature measurement
    pub temperature: f32,
    /// Reference junction temperature measurement
    pub cold_reference: f32,
    /// Fault roll up
    pub fault: bool,
    /// SCV fault
    pub scv: bool,
    /// SCG fault
    pub scg: bool,
    /// OC fault
    pub oc: bool,
}

/// Uncalibrated mesurements from MAX31855
#[allow(dead_code)]
struct Raw {
    /// Thermocouple temperature measurement
    temperature: i16,
    /// Reference junction temperature measurement
    cold_reference: i16,
    /// Fault roll up
    fault: bool,
    /// SCV fault
    scv: bool,
    /// SCG fault
    scg: bool,
    /// OC fault
    oc: bool,
}
