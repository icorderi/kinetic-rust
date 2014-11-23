# A [Kinetic](https://github.com/Seagate/kinetic-protocol) protocol library written in [Rust](http://www.rust-lang.org/).

[![Build Status](https://travis-ci.org/icorderi/kinetic-rust.png?branch=master)](https://travis-ci.org/icorderi/kinetic-rust)

## Getting Started

### Installing Rust

The quicket way to get the current nightly build is to run:

    curl -s https://static.rust-lang.org/rustup.sh | sudo sh

If you want other installation options, visit the official [install](http://www.rust-lang.org/install.html) guide.

### Installing Kinetic-rust from source

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
