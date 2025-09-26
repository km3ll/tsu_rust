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
	 * pod: Option
	 * - Defined and loaded in the prelude
	 */
	let op: Option<String> = find_name(2);
	match op {
		Some(name) => println!("name: {name}"),
		None => println!("name not found!"),
	}
}

pub fn option_if_let_syntax() {
	/**
	 * pod: Options if-let syntax
	 * * - works when you only care about one variant
	 */
	let op1 = find_name(1);
	if let Some(name) = op1 {
		println!("name: {name}")
	}

	let op2 = find_name(2);
	if let None = op2 {
		println!("name not found!")
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn run_option_enum() {
		option_enum();
	}

	#[test]
	fn run_option_if_let_syntax() {
		option_if_let_syntax();
	}
}
