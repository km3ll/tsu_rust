pub fn publishing_crates_io() {
	/**
	 * pod: crates.io
	 * - Changes must be committed to GitHub
	 * - Required fields: description and license
	 * - The name of the package must be unique
	 * - When you publish a package it's permanent
	 * - Cargo uses semantic versioning
	 */
	println!("cargo login <token>");
	println!("cargo publish");
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn run_publishing_crates_io() {
		publishing_crates_io();
		assert!(true)
	}
}
