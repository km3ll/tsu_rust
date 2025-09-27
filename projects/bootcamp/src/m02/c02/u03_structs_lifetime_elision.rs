fn lifetime_elision_() {
	println!("Base");
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn run_() {
		lifetime_elision_();
	}
}
