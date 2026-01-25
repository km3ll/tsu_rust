//! # Rc<T> The Reference Counted Smart Pointer

fn rc_smart_pointer() {
	let n1 = r#"
	pod:
	-
	---"#;
	println!("{n1}");
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn run_rc_smart_pointer() {
		rc_smart_pointer();
	}
}
