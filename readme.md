# tsu-rust

- [1. overview](#1-overview)
- [2. commands](#2-commands)
  - [cargo](#cargo)
  - [rustc](#rustc)
  - [rustfmt](#rustfmt)
  - [rustup](#rustup)
- [3. features](#3-features)

## 1. overview

We follow the learning path described in book: [The Rust Programming Language](https://doc.rust-lang.org/book/title-page.html).

## 2. commands

### `cargo`

```shell
cargo build
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