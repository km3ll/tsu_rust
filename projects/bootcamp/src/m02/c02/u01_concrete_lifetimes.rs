fn concrete_lifetimes_() {
	println!("Base");
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn run_() {
		concrete_lifetimes_();
	}
}
