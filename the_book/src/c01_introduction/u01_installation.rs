//! # Installation

fn installation() {
	let n1 = r#"
	---
	pod: rustup
	- A toolchain multiplexer. It installs and manages many Rust toolchains
	- Presents toolchains through a single set of tools installed to ~/.cargo/bin
	- Installs Rust from the official release channels
	- Makes cross-compiling simpler with binary builds of the standard library

	cmd:
	- `rustc --version`
	- `rustup update` # Update Rust toolchains and rustup
	- `rustup doc` # Open the documentation for the current toolchain
	- `cargo --offline` # Run without accessing the network (local dependencies)
	---"#;
	println!("{n1}");
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn run_installation() {
		installation()
	}
}
