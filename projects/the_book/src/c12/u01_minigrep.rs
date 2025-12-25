fn minigrep_concepts() {
	let n1 = r#"
	pod: grep
	- [G]lobally search a [R]egular [E]xpression and [P]rint
	- Two hyphens in 'cargo run --' to indicate the following arguments are for our program
	---"#;
	println!("{n1}");
}

fn minigrep_() {

}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn run_minigrep_concepts() {
		minigrep_concepts();
	}

	#[test]
	fn run_() {
		minigrep_();
	}
}
