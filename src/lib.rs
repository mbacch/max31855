//! A platform agnostic driver to interface with the MAX31855 (thermocouple digital converter)
//!
//! This driver was built using [`embedded-hal`] traits.

//#![deny(missing_docs)]
//#![deny(warnings)]
//#![feature(unsize)]
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
    pub fn read(&mut self) -> Result<Measurements, E> {
        
        self.cs.set_low();
        
        let mut buffer = [0u8; 4];
        self.spi.transfer(&mut buffer)?;
        
        self.cs.set_high();
        
        // Combine array of u8 to a u32 (MSB to LSB)
        let r = ((buffer[0] as u32) << 24) |
                ((buffer[1] as u32) << 16) |
                ((buffer[2] as u32) <<  8) |
                 (buffer[3] as u32);

        Ok(
            Measurements{
                tc_temperature: r.get_bits(18..32) as u16,
                cj_temperature: r.get_bits(4..15)  as u16,
                fault: r.get_bit(16) as bool,
                scv: r.get_bit(2) as bool,
                scg: r.get_bit(1) as bool,
                oc: r.get_bit(0) as bool,
            }
        )
    }

    // TODO Add interface with calibrated values
    // pub fn read_calibrated(units: Units) -> Result<TBD, E> {
    //     unimplemented!();
    // }

    // TODO Check for faults and returns true if fault exists
    //fn check_fault() -> bool {
    //    unimplemented!();
    //}
}

// TODO Units Enumeration
//pub enum Units {
//    Celsius,
//    Fahrenheit,
//    Kelvin,
//}

/// Thermocouple and Reference Junction Measurements
// TODO Look into modifying structure to include calibrated and uncalibrated data
pub struct Measurements {
    /// Thermocouple temperature measurement
    tc_temperature: u16,  // TODO this needs to be signed. Ok for now if positive Celsius
    /// Reference junction temperature measurement
    cj_temperature: u16,  // TODO this needs to be signed. Ok for now if positive Celsius
    /// Roll-up Fault Bit
    fault: bool,
    /// Short Circuit Fault to Voltage (VCC)
    scv: bool,
    /// Short Circuit Fault to Ground (GND)
    scg: bool,
    /// Open Circuit Fault (No Connections)
    oc:  bool,
}
