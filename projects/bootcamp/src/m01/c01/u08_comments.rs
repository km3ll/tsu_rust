pub fn comments_line() {
	/**
	 * pod: Line comments
	 */
	println!("Line comments")
}

pub fn comments_block() {
	/**
	 * pod: Block comments
	 */
	println!("Block comments")
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn run_comments_line() {
		comments_line();
	}

	#[test]
	fn run_comments_block() {
		comments_block();
	}
}
