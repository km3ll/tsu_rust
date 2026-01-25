fn setup() {
	let n1 = r#"
	pod: VSCode Extensions
	- Rust Analyzer
	- CodeLLDB (native debugger)
	- Even Better TOML
	- Dependi
	- Error Lens
	- TODO Tree
	- GitHub Copilot
	---"#;
	println!("{n1}")
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn run_setup() {
		setup()
	}
}
