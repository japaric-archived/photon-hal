# `particle-hal`

> Low level bindings to [particle]'s [HAL][0] (Hardware Abstraction Layer)

[particle]: https://www.particle.io/
[0]: https://github.com/spark/firmware/tree/develop/hal

## Generating bindings

First, you'll need these dependencies:

- [GCC for ARM Cortex processors][1]
- Nightly Rust
- [bindgen] `generate.sh` will `cargo install` it for you if you don't have it installed already.

[1]: https://github.com/spark/firmware/blob/develop/docs/dependencies.md#1-gcc-for-arm-cortex-processors
[bindgen]: https://github.com/crabtw/rust-bindgen

Next, modify the `modules` array in `generate.sh`, then call:

```
$ bash ./generate.sh
```

You'll get source files like `src/gpio.rs` and `src/delay.rs`. If you added new modules, you'll have
to add `mod $name` items to `src/lib.rs` and add new type definitions to `src/ty.rs`.

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
