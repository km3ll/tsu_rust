fn rc_smart_pointer_() {
	println!("Base");
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn run_() {
		rc_smart_pointer_();
	}
}
