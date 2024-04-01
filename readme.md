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
cargo run
cargo check

cargo build           // target/debug
cargo build --release // target/release
cargo new hello_pod
cargo --help
cargo --version
```

### `rustc`

```shell
rustc main.rs
```

### `rustfmt`

```shell
rustfmt main.rs
cat main.rs
```

### `rustup`

_rustup_ is a command line tool for managing Rust versions and associated tools.

```shell
rustup --version
rustup update

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