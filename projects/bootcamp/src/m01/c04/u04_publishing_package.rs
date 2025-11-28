pub fn publishing_crates_io() {
	let n1 = r#"
	pod: crates.io
	- Changes must be committed to GitHub
	- Required fields: description and license
	- The name of the package must be unique
	- When you publish a package it's permanent
	- Cargo uses semantic versioning
	---"#;
	println!("{n1}");
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn run_publishing_crates_io() {
		publishing_crates_io();
	}
}
