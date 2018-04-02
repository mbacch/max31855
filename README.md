# `max31855`

> A platform agnostic driver to interface with the MAX31855 (Cold-Junction Compensated Thermocouple-to-Digital Converter)

## What works

- Reading the thermocouple, cold junction reference temperature, fault status bits. All readings are raw from the ADC. See TO DO for going forward.

## TODO

- [ ] Add interface for calibrated temperatures (current interface provides raw values from ADC)
- [ ] Additional fault management with the fault status bits. e.g. How to handle interface when faults are present?
- [ ] Need to support signed integers (currently using u16. Need to use i16 and handle signed bit correctly).

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