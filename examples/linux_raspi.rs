
extern crate linux_embedded_hal as hal;
extern crate max31855;

use std::thread;
use std::time::Duration;

use max31855::{Max31855, Units};
use hal::spidev::{self, SpidevOptions};
use hal::{Pin, Spidev};
use hal::sysfs_gpio::Direction;

fn main() {

    /* Configure SPI */
    let mut spi = Spidev::open("/dev/spidev0.0").unwrap();
    let options = SpidevOptions::new()
        .bits_per_word(8)
        .max_speed_hz(1_000_000)
        .mode(spidev::SPI_MODE_0)
        .build();
    spi.configure(&options).unwrap();

    /* Configure Digital I/O Pin to be used as Chip Select */
    let cs = Pin::new(4);
    cs.export().unwrap();
    while !cs.is_exported() {}
    cs.set_direction(Direction::Out).unwrap();
    cs.set_value(1).unwrap();

    let mut max31855 = Max31855::new(spi, cs).unwrap();

    loop {
        println!("{:?}", max31855.read_thermocouple(Units::Fahrenheit).unwrap());
        thread::sleep(Duration::from_millis(1000));
    }
}
