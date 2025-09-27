pub fn features_definition() {
	/**
	 * pod: Cargo features
	 * - Allow to define part of code that are conditionally compiled,
	 *   only if a certain feature is enabled.
	 * - Allow to define optional dependencies
	 * - Reduce compile time and file sizes
	 * - Disabled by default
	 */
	println!("cargo features");
}

pub fn features_cargo() {
	/**
	 * pod: Cargo.toml
	 * - Features have a name and an associated array: other features or optional dependencies.
	 * - Dependencies must be marked as optional
	 */
	println!("Cargo.toml");
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn run_features_definition() {
		features_definition();
	}

	#[test]
	fn run_features_cargo() {
		features_cargo();
	}
}
