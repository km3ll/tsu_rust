pub fn overview() {
    println!("----------");
    println!("Overview");

    /**
     * Basic Components
     * Package
     * - Created when 'cargo new' is executed
     * - A package contains one or more crates
     * - Package rules
     *   - At least 1 crate
     *   - At most 1 library crate
     *   - Any number of binary crates
     *
     * Crate
     * - A tree of modules that produces a library or binary
     *   - Library: code that you can share with other crates
     *   - Binary: code that you can run
     *
     * Module
     * - Control the organization, control and privacy
     */
    println!(" package (cargo new)");
    println!(" ├── library (crate)");
    println!(" │   └── root module (crate root)");
    println!(" │       └── sub-module");
    println!(" └── binary (crate)");
    println!("     └── root module (crate root)");
    println!("         └── sub-module");

    /**
     * Binary Crate
     * - By default a new package comes with a binary crate that
     *   starts at main.rs
     *
     * - Convention: if a main.rs file exists in the source directory,
     *   then it will be the crate root of a binary crate with the same
     *   name as the package.
     *
     * - Convention: if a bin.rs file exists in the source directory,
     *   then it will be the crate root of a library crate.
     *
     * - The package can contain both, which is a common pattern for
     *   CLI applications
     *
     * - For more than 1 binary crate we should create a subdirectory
     *   within the src directory called 'bin'
     */
    println!(" base_package");
    println!(" > bin");
    println!("   > another_main.rs");
    println!(" > lib.rs");
    println!(" > main.rs");

}