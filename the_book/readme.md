# the_book

[The Rust Programming Language](https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html#handling-potential-failure-with-result)

## content

```
The Rust Programming Language
- c01 Getting Started
  - u01 Installation
  - u02 Hello, World!
  - u03 Hello, Cargo!
  - u04 Hello, Tests!
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
  - u01 Packages and Crates
  - u02 Defining Modules to Control Scope and Privacy
  - u03 Paths for Referring to an Item in the Module Tree
  - u04 Bringing Paths Into Scope with the use keyword
  - u05 Separating Modules into Different Files
- c08 Common Collections
  - u01 Storing Lists of Values with Vectors
  - u02 Storing UTF-8 Encoded Text with Strings
  - u03 Storing Keys with Associated Values in Hash Maps
- c09 Error Handling
  - u01 Unrecoverable Errors with panic!
  - u02 Recoverable Errors with Result
  - u03 To panic! or Not to panic!
- c10 Generic Types, Traits, and Lifetimes
  - u01 Generic Data Types
  - u02 Defining Shared Behavior with Traits
  - u03 Validating References with Lifetimes
- c11 Writing Automated Tests
  - u01 How to Write Tests
  - u02 Controlling How Tests Are Run
  - u03 Test Organization
- c12 An I/O Project: Building a Command Line Program
  - u01 Accepting Command Line Arguments
  - u02 Reading a File
  - u03 Refactoring to Improve Modularity and Error Handling
  - u04 Adding Functionality with Test Driven Development (TDD)
  - u05 Working with Environment Variables
  - u06 Redirecting Errors to Standard Error
- c13 Functional Language Features: Iterators and Closures
  - u01 Closures
  - u02 Processing a Series of Items with Iterators
  - u03 Improving Our I/O Project
  - u04 Performance in Loops vs Iterators
- c14 More about Cargo and Crates.io
  - u01 Customizing Builds with Release Profiles
  - u02 Publishing a Crate to Crates.io
  - u03 Cargo Workspaces
- c15 Smart Pointers
  - u01 Using Box<T> to Point to Data on the Heap
  - u02 Treating Smart Pointers Like Regular References
  - u03 Running Code on Cleanup with the Drop Trait
  - u04 Rc<T> The Reference Counted Smart Pointer
  - u05 RefCell<T> and the Interior Mutability Pattern
  - u06 Reference Cycles Can Leak Memory
- c16 Fearless Concurrency
```

## commands

```bash
# project
cargo clean
cargo fmt
cargo build

# target/debug
cargo run
# target/release
cargo run --release

cargo run --bin the_book
cargo run --bin app

# tests
cargo test -- --nocapture
cargo test --help
cargo test -- --help
cargo test -- --test-threads=1
cargo test -- --show-output
cargo test run_test_control_definition
cargo test -- --ignored
cargo test -- --include-ignored
cargo test --test integration_test

# documentation
# target/doc/the_book/index.html
cargo doc --open 

# crates.io
cargo login
cargo publish
cargo yank --vers 1.0.1
cargo yank --vers 1.0.1 --undo

# workspaces
cargo run -p adder
cargo test -p add_one

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
- E0005 Patterns used to bind names must be irrefutable
- E0040 It is not allowed to manually call destructors in Rust
- E0072 A recursive type has infinite size because it doesn't have an indirection
- E0106 A lifetime is missing from a type
- E0133 Unsafe code was used outside of an unsafe block
- E0277 Tried to use a type which doesn't implement some trait in a place which expected that trait
- E0284 The compiler is unable to unambiguously infer the return type of a function
- E0308 Expected type did not match the received type
- E0369 A binary operation was attempted on a type which doesn't support it
- E0382 A variable was used after its contents have been moved elsewhere
- E0384 An immutable variable was reassigned
- E0391 A type dependency cycle has been encountered
- E0433 An undeclared crate, module, or type was used
- E0499 A variable was borrowed as mutable more than once
- E0502 A variable already borrowed with a certain mutability was borrowed again with a different mutability
- E0507 A borrowed value was moved out
- E0515 A reference to a local variable was returned
- E0596 Tried to mutably borrow a non-mutable variable
- E0597 A value was dropped while it was still borrowed
- E0603 A private item was used outside its scope
- E0614 Attempted to dereference a variable which cannot be dereference
- E0752 The entry point of the program was marked as async
- E0790 You need to specify a specific implementation of the trait in order to call the method
```

## references

- [Benchmark Tests](https://doc.rust-lang.org/unstable-book/library-features/test.html)
- [Cargo documentation](https://doc.rust-lang.org/cargo/)
- [Cargo profiles](https://doc.rust-lang.org/cargo/reference/profiles.html)
- [Clippy](https://doc.rust-lang.org/stable/clippy/index.html)
- [mdBook](https://rust-lang.github.io/mdBook/index.html)
- [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- [Rust By Example](https://doc.rust-lang.org/rust-by-example/index.html)
- [Rust Playground](https://play.rust-lang.org)
- [Semantic Versioning 2.0.0](https://semver.org/)
- [The Linux Foundation’s SPDX License List](https://spdx.org/licenses/)
- [The Rust Programming Language](https://doc.rust-lang.org/book/title-page.html)
- [The rustc Book](https://doc.rust-lang.org/rustc/what-is-rustc.html)
  - [tests](https://doc.rust-lang.org/rustc/tests/index.html)
- [The Rust Standard Library](https://doc.rust-lang.org/stable/std/)
  - [Module collections](https://doc.rust-lang.org/std/collections/index.html)
  - [Struct Vec](https://doc.rust-lang.org/std/vec/struct.Vec.html)
- [The Rustonomicon](https://doc.rust-lang.org/nomicon/intro.html)