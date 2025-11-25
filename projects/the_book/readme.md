# the_book

[The Rust Programming Language](https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html#handling-potential-failure-with-result)

## content

```
The Rust Programming Language
- c01 Getting Started
  - u01 Installation
  - u02 Hello, World!
  - u03 Hello, Cargo!
- c02 Programming a Guessing Game
- c03 Common Programming Concepts
  - u01 Variables and Mutability
  - u02 Data Types
```

## commands

```bash
# project
cargo clean
cargo fmt
cargo build
cargo test -- --nocapture
# target/debug
cargo run
# target/release
cargo run --release

cargo run --bin the_book
cargo run --bin app

# projects
cargo new package_name # binary crate
cargo new --lib package_name # library crate
cargo new hello_cargo --vcs=git # existing repository

cargo init # create a Cargo.toml file 
cargo install cargo-modules
cargo install cargo-make
cargo --version

rustc --version
rustup update
rustup doc
```

## references

- [Cargo documentation](https://doc.rust-lang.org/cargo/)