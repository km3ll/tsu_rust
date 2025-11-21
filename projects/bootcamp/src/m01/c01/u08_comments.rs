pub fn comments() {
	let n1 = r#"
	pod: Comments
	- line
	- block
	-----"#;
	println!("{n1}");
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn run_comments() {
		comments()
	}
}
