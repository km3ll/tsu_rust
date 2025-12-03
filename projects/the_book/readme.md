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
  - u02 An Example Program Using Structs
  - u03 Method Syntax
- c06 Enums and Pattern Matching
  - u01 Defining an Enum
  - u02 The match Control Flow Construct
  - u03 Concise Control Flow with if let and let else
- c07 Managing Growing Projects with Packages, Crates, and Modules
  - u01 - Packages and Crates
  - u02 - Defining Modules to Control Scope and Privacy
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

- E0004 Non-exhaustive patterns
- E0106 A lifetime is missing from a type
- E0277 Tried to use a type which doesn't implement some trait in a place which expected that trait
- E0284 The compiler is unable to unambiguously infer the return type of a function
- E0308 Expected type did not match the received type
- E0382 A variable was used after its contents have been moved elsewhere
- E0384 An immutable variable was reassigned
- E0499 A variable was borrowed as mutable more than once
- E0502 A variable already borrowed with a certain mutability was borrowed again with a different mutability
- E0596 Tried to mutably borrow a non-mutable variable
- E0603 A private item was used outside its scope.
```

## references

- [The Rust Programming Language](https://doc.rust-lang.org/book/title-page.html)
- [Rust By Example](https://doc.rust-lang.org/rust-by-example/index.html)
- [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- [Cargo documentation](https://doc.rust-lang.org/cargo/)