fn hello_cargo() {
	let n1 = r#"
	pod: 1.3. Hello, Cargo!
	- Cargo is Rust's build system and package manager
	pod: TOML
	- Tom's Obvious, Minimal Language (configurational format)
	cmd:
	- cargo --version
	-----"#;
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
