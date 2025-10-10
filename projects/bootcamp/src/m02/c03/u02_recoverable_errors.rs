fn recoverable_errors_() {
	println!("Base");
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn run_() {
		recoverable_errors_();
	}
}
