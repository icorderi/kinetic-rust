# A [Kinetic] protocol library written in [Rust].

[Kinetic]: https://github.com/Seagate/kinetic-protocol
[Rust]: http://www.rust-lang.org/

[![Travis](https://img.shields.io/travis/icorderi/kinetic-rust.svg)](https://travis-ci.org/icorderi/kinetic-rust)
[![Crates.io](https://img.shields.io/crates/v/kinetic.svg)](https://crates.io/crates/kinetic)
[![Crates.io](https://img.shields.io/crates/l/kinetic.svg)](https://github.com/icorderi/kinetic-rust/blob/master/LICENSE/mit.md)

## Getting Started

### Installing Rust

If you don't have Rust yet the quicket way to get the current nightly build is to run:

    curl -s https://static.rust-lang.org/rustup.sh | sudo sh -s -- --channel=nightly

If you want other installation options, visit the official [install] guide.

[install]: http://www.rust-lang.org/install.html

### Adding the Kinetic-rust dependency

If you are using [Cargo] and want to use the latest stable Kinetic-rust [crate] available at [crates.io] add this to your `Cargo.toml`:

```toml
[dependencies.kinetic-rust]
```

To get the dependency to be linked directly to the GitHub repo then add this instead:

```toml
[dependencies.kinetic-rust]
    git = "https://github.com/icorderi/kinetic-rust.git"
```

> **Note:** For more information on handling [dependencies] check the official cargo site.

[Cargo]: http://doc.crates.io/index.html
[crate]: https://crates.io/crates/kinetic-rust
[crates.io]: https://crates.io/
[dependencies]: http://doc.crates.io/guide.html#adding-dependencies

### Importing Kinetic-rust

To import Kinetic-rust from your code add this statement:

```rust
extern crate kinetic; // depend on the kinetic-rust library
```

### [Optional] Installing Kinetic-rust from source

    git clone https://github.com/icorderi/kinetic-rust.git
    cd kinetic-rust
    cargo build --release

> **Note:** The `--release` flag will compile the code with the optimizations turned on.

Additionally you can run the tests or compile the documentation locally:

    cargo test
    cargo doc

The local HTML documentation will be available at `./target/doc/kinetic/index.html`.

> **Note:** Some tests will require you to have a Kinetic device available. You can use the [simulator] if you don't have real devices.

[simulator]: https://github.com/seagate/kinetic-java

## Documentation

If you need help don't forget to checkout the online [documentation] for the library.

[documentation]: http://icorderi.github.io/kinetic-rust/doc/kinetic

## Contributing

Get involved with the [issues] or submit a [PR].

[issues]: https://github.com/icorderi/kinetic-rust/issues
[PR]: https://github.com/icorderi/kinetic-rust/pulls

## License

This project is licensed under The MIT License (MIT)
* [Markdown](LICENSE/mit.md) version
* [Original](LICENSE/mit.txt) version
