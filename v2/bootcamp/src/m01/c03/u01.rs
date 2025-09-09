pub fn structs_() {
	println!("Base");
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn run_() {
		structs_();
		assert!(true)
	}
}
