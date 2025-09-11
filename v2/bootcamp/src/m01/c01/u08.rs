pub fn comments_line() {
	// line comments
	println!("Line comments")
}

pub fn comments_block() {
	/**
	 * block comments
	 */
	println!("Block comments")
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn run_comments_line() {
		comments_line();
		assert!(true)
	}

	#[test]
	fn run_comments_block() {
		comments_block();
		assert!(true)
	}
}
