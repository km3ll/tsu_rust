fn hello_world() {
	let n1 = r#"
	pod: Hello, World!
	- Rust files end with .rs extension
	- Naming convention is snake case for multiple words
	- Function main() in the entry point of Rust executables
	---
	cmd:
	- rustc main.rs
	- ./main
	- cargo test -- --nocapture
	---
	pod: Macros
	- A way to write code that generates code
	- Using a ! means we're calling a macro
	---
	pod: Ahead-Of-Time Compiled Language
	- You can give the executable to someone else, and they can run it, even without having Rust installed
	---"#;
	println!("{n1}")
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn run_hello_world() {
		hello_world()
	}
}
