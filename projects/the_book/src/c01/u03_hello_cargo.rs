fn hello_cargo() {
	let n1 = r#"
	pod: 1.3. Hello, Cargo!
	- Cargo is Rust's build system and package manager

	pod: TOML
	- Tom's Obvious, Minimal Language (configuration format)

	pod: Crates
	- Packages of code (dependencies

	pod: Cargo.lock
	- Keeps track of the exact versions of dependencies

	cmd:
	- cargo --version
	- cargo new <name> (initializes git)
	- cargo new <name> --vcs=git (uses git)
	- cargo build (target/debug/<name>)
	- ./target/debug/<name>
	- cargo build --release (optimized, target/release/<name>)
	- cargo run
	- cargo check (faster, compiles, doesn't produce executable)
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
