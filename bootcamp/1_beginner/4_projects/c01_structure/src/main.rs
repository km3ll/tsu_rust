// Default binary crate that starts at main.rs

// Cargo follows the convention that if a main.rs file exists in the source directory
// then it will be the crate root of a binary crate with the same names as the package.
// In this case: c01_structure

// If we have both a main.rs and a lib.rs file in our source directory, then our
// package has two crates: a binary crate and a library crate.
// This is a common pattern for CLI applications.

// Packages can have at most one library crate, but any amount of binary crates.
// If we want to create more binary crates, we could create a subdirectory within
// the source directory called bin.

// Our package has three crates:
// - c01_structure: binary
// - c01_structure: library
// - another: binary

// Command cargo run won't work anymore
// Error: could not determine which binary to run.
// cargo run --bin another

// If you'd like a library crate you can specify the --lib flag

fn main() {
    println!("Hello, world!");
}