# build-tool-config

[![Build Status](https://travis-ci.org/matthieusb/build-tool-config.svg?branch=develop)](https://travis-ci.org/matthieusb/build-tool-config)

TODO Add project description

## Installation 

Coming eventually

## Development

This project has been built using Rust 1.26.2

## Building the project

To build it classically :

```bash
cargo build
```

You can also install the following dependencies:

```bash
cargo install cargo-watch
```

And then to build continually in separate terminals or launch tests:

```bash
cargo watch
cargo watch -x test
```

## Generating the documentation

To generate and open this project's documentation:

```bash
cargo doc --open
```