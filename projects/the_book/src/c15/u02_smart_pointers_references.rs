//! # Treating Smart Pointers Like Regular References

fn references() {
	let n1 = r#"
	pod:
	-
	---"#;
	println!("{n1}");
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn run_references() {
		references();
	}
}
