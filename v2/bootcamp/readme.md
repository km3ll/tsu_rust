# bootcamp

## content

```
bootcamp
- m01 beginner
  - c01 Get up and running fast
    - u01 Setup
    - u02 Hello world
    - u03 Variables
    - u04 Data types
    - u05 Constants and statics
    - u06 Functions
    - u07 Flow control
    - u08 Comments
  - c02 How Rust achieves memory safety
    - u01 Memory regions
    - u02 Ownership part 1
    - u03 Ownership part 2
    - u04 Borrowing
    - u05 Slices
    - u06 Strings
  - c03 Build your own data types
    - u01 Structs
    - u02 Implementation blocks
    - u03 Tupple structs
    - u04 Enums
    - u05 Matching
    - u06 Option
    - u07 Result
    - u08 Vector
  - c04 How to structure your projects
    - u01 Project structure overview
      - p01_basic
    - u02 Modules 
      - bootcamp_p02
      - bootcamp_p03
      - bootcamp_p04
    - u03 External dependencies
    - u04 Publishing your package
- m02 intermediate
- m03 advanced
- m04 masterclass
- m05 after bootcamp
```

## commands

```bash
cargo clean build
cargo new package_name       # binary crate
cargo new --lib package_name # library crate

# target/debug
cargo run
cargo run --bin base_package
cargo run --bin another_main

# target/release
cargo run --release

# modules
cargo-modules modules structure

# plugins
cargo install cargo-modules

docker build .
```
