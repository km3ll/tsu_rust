pub fn greet() {
	let n1 = r#"
	pod: Macros
	- println!()
	-----"#;
	println!("{n1}");
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn run_greet() {
		greet();
	}
}
