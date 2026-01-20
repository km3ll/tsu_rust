fn profiles() {
	let n1 = r#"
	pod: Release Profiles
	- Predefined, customizable profiles with different configurations
	- Cargo uses 'dev' profile when you run 'cargo build'
	- Cargo uses 'release' profile when you run 'cargo build --release'
	- Cargo uses the defaults for a profile plus our customizations
	---"#;
	println!("{n1}");

	println!(
		"
	[profile.dev]
	opt-level = 0
	"
	);

	println!(
		"
	[profile.release]
	opt-level = 3
	"
	);
}

fn profiles_optimization() {
	let n1 = r#"
	pod: opt-level
	- Controls the number of optimizations Rust will apply to your code
	---"#;
	println!("{n1}");
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn run_profiles() {
		profiles();
	}

	#[test]
	fn run_profiles_optimization() {
		profiles_optimization()
	}
}
