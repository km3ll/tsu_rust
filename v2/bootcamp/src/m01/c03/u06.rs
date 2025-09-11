fn find_name(id: u32) -> Option<String> {
	let name = String::from("Ferris");
	if (id == 1) {
		Some(name)
	} else {
		None
	}
}

pub fn option_enum() {
	/**
	 * pod: Option is defined and loaded in the prelude
	 */
	let rs: Option<String> = find_name(2);
	match rs {
		Some(name) => println!("name: {name}"),
		None => println!("name not found!"),
	}
}

pub fn option_if_let_syntax() {
	/**
	 * pod: Options if-let syntax
	 * * - works when you only care about one variant
	 */
	let rs1 = find_name(1);
	if let Some(name) = rs1 {
		println!("name: {name}")
	}

	let rs2 = find_name(2);
	if let None = rs2 {
		println!("name not found!")
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn run_option_enum() {
		option_enum();
		assert!(true)
	}

	#[test]
	fn run_option_if_let_syntax() {
		option_if_let_syntax();
		assert!(true)
	}
}
