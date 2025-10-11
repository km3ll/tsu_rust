fn propagating_errors_() {
	println!("Base");
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn run_() {
		propagating_errors_();
	}
}
