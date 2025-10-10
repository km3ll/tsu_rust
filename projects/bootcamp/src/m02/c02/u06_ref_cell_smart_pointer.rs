fn ref_cell_smart_pointer_() {
	println!("Base");
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn run_() {
		ref_cell_smart_pointer_();
	}
}
