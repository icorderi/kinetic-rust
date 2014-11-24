# A [Kinetic](https://github.com/Seagate/kinetic-protocol) protocol library written in [Rust](http://www.rust-lang.org/).

[![Build Status](https://travis-ci.org/icorderi/kinetic-rust.png?branch=master)](https://travis-ci.org/icorderi/kinetic-rust)

## Getting Started

### Installing Rust

The quicket way to get the current nightly build is to run:

    curl -s https://static.rust-lang.org/rustup.sh | sudo sh

If you want other installation options, visit the official [install](http://www.rust-lang.org/install.html) guide.

### Adding the Kinetic-rust dependency
If you want to use the latest stable [crate](https://crates.io/crates/kinetic-rust) version available at [crates.io](https://crates.io/) add this to your `Cargo.toml`:
```toml
[dependencies.kinetic-rust]
```
If you are using [Cargo](http://doc.crates.io/index.html) and want the dependency to be linked directly to the GitHub repo then add this instead:
```toml
[dependencies.kinetic-rust]
git = "https://github.com/icorderi/kinetic-rust.git"
```
_Note: For more information on handling [dependencies](http://doc.crates.io/guide.html#adding-dependencies) check the official cargo site._


### Importing Kinetic-rust
To import Kinetic-rust from your code add this statement:
```rust
extern crate kinetic; // depend on the kinetic-rust library
```

### [Optional] Installing Kinetic-rust from source

    git clone https://github.com/icorderi/kinetic-rust.git
    cd kinetic-rust
    cargo build --release

_Note: The `--release` flag will compile the code with the optimizations turned on._

Additionally you can run the tests or compile the documentation locally:

    cargo test
    cargo doc

The local HTML documentation will be available at `./target/doc/kinetic/index.html`.

_Note: Some tests will require you to have a Kinetic device available. You can use the [simulator](https://github.com/seagate/kinetic-java) if you don't have real devices._

## Documentation

If you need help don't forget to checkout the online [documentation](http://icorderi.github.io/kinetic-rust/doc/kinetic/) for the library.

## Contributing

Get involved with the [issues](https://github.com/icorderi/kinetic-rust/issues) or submit a [PR](https://github.com/icorderi/kinetic-rust/pulls).

## License
This project is licensed under The MIT License (MIT)
* [Markdown](LICENSE/mit.md) version
* [Original](LICENSE/mit.txt) version
