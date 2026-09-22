//! # Hello, Cargo!

fn hello_cargo() {
	let n1 = r#"
	---
	pod: Cargo
	- Rust's build system and package manager
	
	pod: TOML
	- Tom's Obvious, Minimal Language
	- A config file format for humans
	
	pod: Crate
	- The smallest amount of code that the Rust compiler considers at a time
	- Crates can contain modules
	
	pod: Binary Crate
	- Programs you can compile to an executable that you can run
	
	pod: Library Crate
	- Don't have a `main` function, and they don't compile to an executable
	- They define functionality intented to be shared
	
	pod: Cargo.lock
	- Keeps track of the exact versions of dependencies

	cmd:
	- `cargo --version`
	- `cargo new <name>` # initializes git
	- `cargo new <name> --vcs=git` # uses git
	- `cargo build` # ./target/debug/<name>
	- `cargo build --release` # optimized, ./target/release/<name>
	- `cargo run` # doestn't rebuild if files haven't changed
	- `cargo check` # faster, compiles, doesn't produce executable
	---"#;
	println!("{n1}")
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn run_hello_cargo() {
		hello_cargo();
	}
}
