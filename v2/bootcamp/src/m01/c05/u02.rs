pub fn workspace_() {
	println!("Base");
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn run_() {
		workspace_();
		assert!(true)
	}
}
