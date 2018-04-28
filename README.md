# `max31855`

A platform agnostic driver to interface with the MAX31855 (Cold-Junction Compensated Thermocouple-to-Digital Converter)

## What works

- Reading the thermocouple, cold junction reference temperature, fault status bits.
- Interface to read the hot junction reference in Counts, Fahrenheit, Celsius, and Kelvin.
- Currently testing with the MAX31855K and a Type K Thermocouple (Using the Sparkfun Thermocouple Breakout)

## TODO

- [x] Add interface for calibrated temperatures (current interface provides raw values from ADC)
- [ ] Additional fault management with the fault status bits. e.g. How to handle interface when faults are present? Partially done, need to pass NAN when fault exists
- [x] Need to support signed integers (currently using u16. Need to use i16 and handle signed bit correctly).
- [ ] Test on Raspberry Pi (currently testing on the DISCOVERY)
- [ ] Finish read_all interface
- [ ] Perform fault testing (ensure fault bits work correctly)

## Examples

In work

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the
work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
