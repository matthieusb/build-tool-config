# build-tool-config

[![Build Status](https://travis-ci.org/matthieusb/build-tool-config.svg?branch=develop)](https://travis-ci.org/matthieusb/build-tool-config)
[![Coverage Status](https://coveralls.io/repos/github/matthieusb/build-tool-config/badge.svg?branch=develop)](https://coveralls.io/github/matthieusb/build-tool-config?branch=develop)

TODO Add project description

## Pre-requisites

In order to work with this project you should use a **nightly channel** of rust. This is needed to use the *tarpaulin* cargo package.


You might need to install the following dependencies:

```
sudo apt install libssl-dev pkg-config cmake zlib1g-dev
sudo apt install cmake g++ pkg-config jq libcurl4-openssl-dev libelf-dev libdw-dev binutils-dev libiberty-dev
```

And then you can install the cargo utilities:

```
cargo install cargo-watch
cargo install cargo-kcov
cargo kcov --print-install-kcov-sh | sh
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
cargo watch -x "test -- --test-threads=1"
```

For now *assert_cli* `with_env` crashes so we can't use it.  Maybe with a more complete version of *assert_cmd*,, we'll be able to use it.

### Generating coverage reports (Linux only)

#### Using kcov (Local usage)

For now this does not seem to work

#### Using tarpaulin (Used for CI generation)

Tarpaulin is used for the CI coverage reports generation but can also be used locally (not the best though)
Use the following command (*This will clean and rebuild the whole project, be careful*):

```
cargo tarpaulin -v - --test-threads=1
```

You can also output the tests as xml:

```
cargo tarpaulin --out Xml -v - --test-threads=1
```

And then for better vizualisation, you can use [pycobertura](https://pypi.org/project/pycobertura/) this way:

```
pycobertura show coverage.xml
```

## Generating the documentation

To generate and open this project's documentation:

```bash
cargo doc --open
```