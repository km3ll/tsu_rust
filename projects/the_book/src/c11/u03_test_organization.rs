fn test_organization_definition() {
	let n1 = r#"
	pod: Unit Tests
	- Small and more focused
	- Test one module or unit of code in isolation at a time
	- The convention is to create a module name tests in each file, annotated with #[cfg(test)]
	- Not included in the compiled artifact
	- Rust allow to test private interfaces, because items in child modules can use the items in their ancestor modules
	---
	pod: Integration Tests
	- Entirely external to your library and use your code in the same way any other external code would
	- Use only the public interface and potentially exercising multiple modules per test
	- Each file in the tests directory is a separate crate
	- If any test in a section fails, the following section will not be run
	- Files in subdirectories of the 'tests' directory don't get compiled as separate crates or have sections in the test output
	- If our project doesn't have a 'src/lib.rs' file, we can't create integration tests
	  - Only binary crates (src/lib) expose functions that other crates can use
	  - Binary crates (src/main) are meant to run on their own
	---
	cmd:
	- cargo test --test integration_test
	---"#;
	println!("{n1}");
}

fn test_organization_() {}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn run_test_organization_definition() {
		test_organization_definition();
	}

	#[test]
	fn run_() {
		test_organization_();
	}
}
