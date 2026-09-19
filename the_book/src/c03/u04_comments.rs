//! # Comments

fn comments() {
	// This is a comment
	println!("Hello, pod!");
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn run_comments() {
		comments();
	}
}
