# LibAFL Mutators Derive

This crate provides derive macros for implementing mutators in LibAFL. It allows you to automatically implement the `Mutator` trait for your structs.

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
libafl_mutators_derive = "0.11.2"
```

Then you can use the derive macro:

```rust
use libafl_mutators_derive::Mutator;

#[derive(Mutator)]
struct MyStruct {
    name: String,
    count: u32,
    data: Vec<u8>,
}
```

The derive macro will automatically implement the `Mutator` trait for your struct, providing intelligent mutations for each field based on its type.

## Features

- Type-specific mutations for common types (String, Vec, numbers)
- Support for nested structs
- Configurable mutation probability
- Error handling and propagation

## License

Licensed under either of:

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)
