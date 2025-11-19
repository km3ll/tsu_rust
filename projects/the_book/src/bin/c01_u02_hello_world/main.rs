/*
pod:
- Rust files end with .rs extension
- Naming convention is snake case for multiple words
- main() is the entry point of Rust executables
  - rustc main.rs
  - ./main
- Macros are a way to write code that generates code
  - Using a '!' means we're calling a macro
- Rust is an ahead-of-time compiled language
  - You can give the executable to someone else, and they can
  - run it, even without having rust installed
 */
fn main() {
	println!("Hello, world!")
}
