# `bm1397-protocol`

[![Crate][crate-image]][crate-link]
[![Docs][docs-image]][docs-link]
![MIT licensed][license-image]

Rust driver for the BM1397 Bitcoin Mining Asic.

It is independant from any embedded-hal traits as the async traits are not yet available for serial.

## Resources

- [BM1397 documentation][documentation]

## License

Dual licensed under your choice of either of:

 - Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
 - MIT license ([LICENSE-MIT](LICENSE-MIT) or
   http://opensource.org/licenses/MIT)

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.

[crate-image]: https://img.shields.io/crates/v/bm1397-protocol.svg
[crate-link]: https://crates.io/crates/bm1397-protocol
[docs-image]: https://docs.rs/bm1397-protocol/badge.svg
[docs-link]: https://docs.rs/bm1397-protocol/
[license-image]: https://img.shields.io/badge/license-Apache2.0/MIT-blue.svg
[documentation]: https://github.com/skot/BM1397/blob/master/registers.md