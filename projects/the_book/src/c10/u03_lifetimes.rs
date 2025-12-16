fn lifetimes_() {
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
	fn run_() {
		lifetimes_();
	}
}
