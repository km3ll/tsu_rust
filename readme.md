# tsu_rust

- [1. overview](#1-overview)
- [2. commands](#2-commands)
  - [cargo](#cargo)
  - [rustc](#rustc)
  - [rustfmt](#rustfmt)
  - [rustup](#rustup)
- [3. features](#3-features)
- [4. bibliography](#4-bibliography)

## 1. overview

We follow the learning path described in _The Rust Programming Language_.

## 2. commands

### `cargo`

```shell
cargo clean
cargo check
cargo run
cargo run --bin another
cargo add rand
cargo add random

cargo build           // target/debug
cargo build --release // target/release
cargo new hello_pod
cargo new --lib hello_lib

cargo --help
cargo --version

cargo install cargo-modules
cargo install cargo-watch

# workspace
cargo new draw --lib --vcs none # no git repository
cargo build -p <package-name> # target a package

# generate doc
cargo doc
cargo doc --open

# benchmarks
cargo bench

cargo-modules structure

# environment

## errors
RUST_BACKTRACE=1
RUST_BACKTRACE=full

## logging
RUST_LOG=off|error cargo run

```

### `rustc`

```shell
rustc main.rs
rustc --explain E0384
```

### `rustfmt`

```shell
rustfmt main.rs
cat main.rs
```

### `rustup`

_rustup_ is a command line tool for managing Rust versions and associated tools.

```shell
rustup default stable
rustup update
rustup --version

# Local copy of Rust documentation
rustup doc
```

## 3. features

### 3.1. actions

A GitHub workflow is defined in file `.github/workflows/rust.yml`

## 4. bibliography

- [The Rust Programming Language](https://doc.rust-lang.org/book/title-page.html)
- [TOML - Tom's Obvious Minimal Language](https://toml.io/en/)
- [The Cargo Book](https://doc.rust-lang.org/cargo/)
- [Module std::prelude](https://doc.rust-lang.org/std/prelude/index.html)
- [Semantic Versioning 2.0.0](https://semver.org/)
- [Crates.io](https://crates.io/)
