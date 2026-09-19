//! # 

fn base_() {
	let n1 = r#"
	---"#;
	println!("{n1}");
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn run_() {
		base_();
	}
}
