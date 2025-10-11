fn multiple_error_types_() {
	println!("Base");
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn run_() {
		multiple_error_types_();
	}
}
