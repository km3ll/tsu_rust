fn deref_coercion_() {
	println!("Base");
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn run_() {
		deref_coercion_();
	}
}
