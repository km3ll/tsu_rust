fn installation() {
	let n1 = r#"
	pod: 1.1. Installation
	- rustup
	- linker (C compiler)

	cmd:
	- rustc --version
	- rustup update
	- rustup doc (local documentation)
	- cargo --offline (cached dependencies)
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
