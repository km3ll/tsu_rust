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
	fn comments_line_runs() {
		comments_line();
		assert!(true)
	}

	#[test]
	fn comments_block_runs() {
		comments_block();
		assert!(true)
	}
}
