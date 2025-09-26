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
      - p02_in_line
      - p03_mod_file
      - p04_mod_folder
    - u03 External dependencies
    - u04 Publishing your package
  - c05 Structuring larger projects
    - u01 Cargo features
      - p05_features_lib
      - p06_features_bin
    - u02 Cargo workspaces
      - p07_workspace
  - c06 How to test and document your code
    - u01 Unit tests
    - u02 Integration tests
      - p08_tests
    - u03 Documentation
    - u04 Benchmark testing
      - p09_benchmark
- m02 intermediate
  - c01 Polymorphism with generics and traits
    - u01 Generics
    - u02 Traits
    - u03 Trait bounds
    - u04 Super traits
    - u05 Trait objects
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

# workspaces
cargo build -b blog_api
cargo build -b blog_web
cargo build -b blog_shared

# documentation
cargo doc
cargo doc --open

# benchmarks
cargo bench

docker build .
```

## benchmarks

```
# bubble_sort

Sorting algorithm       time:   [2.7247 ns 2.7290 ns 2.7372ns]                               
Found 13 outliers among 100 measurements (13.00%)
  5 (5.00%) high mild
  8 (8.00%) high severe

# bubble_sort

Sorting algorithm       time:   [2.7353 ns 2.7390 ns 2.7428 ns]                               
                        change: [-0.1197% +0.1971% +0.4848%] (p = 0.22 > 0.05)
                        No change in performance detected.
Found 3 outliers among 100 measurements (3.00%)
  1 (1.00%) high mild
  2 (2.00%) high severe

# selection_sort

Sorting algorithm       time:   [8.5274 ns 8.5505 ns 8.5719 ns]                               
                        change: [+210.36% +211.47% +212.65%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 3 outliers among 100 measurements (3.00%)
  1 (1.00%) low mild
  1 (1.00%) high mild
  1 (1.00%) high severe
```