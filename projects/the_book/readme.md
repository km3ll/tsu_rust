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
  - u03 Functions
  - u04 Comments
  - u05 Control Flow
- c04 Understanding Ownership
  - u01 What is Ownership?
  - u02 References and Borrowing
  - u03 The Slice Type
- c05 Using Structs to Structure Related Data
  - u01 Defining and Instantiating Structs
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

rustc --explain E0382
rustc --version
rustup update
rustup doc
```

## Error Codes

```
rustc --explain <code>

- E0106 A lifetime is missing from a type
- E0284 The compiler is unable to unambiguously infer the return type of a function
- E0308 Expected type did not match the received type
- E0382 A variable was used after its contents have been moved elsewhere
- E0384 An immutable variable was reassigned
- E0499 A variable was borrowed as mutable more than once
- E0502 A variable already borrowed with a certain mutability was borrowed again with a different mutability
- E0596 Tried to mutably borrow a non-mutable variable
```

## references

- [Cargo documentation](https://doc.rust-lang.org/cargo/)