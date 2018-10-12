# build-tool-config

[![Build Status](https://travis-ci.org/matthieusb/build-tool-config.svg?branch=develop)](https://travis-ci.org/matthieusb/build-tool-config)
[![Coverage Status](https://coveralls.io/repos/github/matthieusb/build-tool-config/badge.svg?branch=develop)](https://coveralls.io/github/matthieusb/build-tool-config?branch=develop)

TODO Add project description

## Pre-requisites

In order to work with this project you should use a **nightly channel** of rust. This is needed to use the *tarpaulin* cargo package.


You might need to install the following dependencies:

```
sudo apt install libssl-dev pkg-config cmake zlib1g-dev
```

And then you can install the cargo utilities:

```
cargo install cargo-watch
RUSTFLAGS="--cfg procmacro2_semver_exempt" cargo install cargo-tarpaulin
```

## Building the project

To build it classically :

```bash
cargo build
```

To build continually:

```bash
cargo watch -x build
```

## Testing the project

### Main recommandations

**ATTENTION:** Testing uses environment variables, which are shared resources. So we can't allow the test to run in parallel (for now).

To launch tests, you should use:

```
cargo test -- --test-threads=1
```

Or to continually test:

```
cargo watch -x 'test -- --test-threads=1'
```

For now *assert_cli* `with_env` crashes so we can't use it.  Maybe with a more complete version of *assert_cmd*,, we'll be able to use it.

### Generating coverage reports

Use the following command (This will clean and rebuild the whole project, be careful):

```
cargo tarpaulin -v - --test-threads=1
```


## Generating the documentation

To generate and open this project's documentation:

```bash
cargo doc --open
```