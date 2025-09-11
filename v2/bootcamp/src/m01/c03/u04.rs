use serde::Serialize;

/**
 * pod: Enums allow to define a type enumerating its variance
 */
#[derive(Debug)]
enum Category {
	Books,
	Clothing,
	Electronics,
}

#[derive(Debug)]
struct Product {
	name: String,
	category: Category,
}

/**
 * pod: Enum variants can have data associated with them
 */
#[derive(Debug, Serialize)]
enum Command {
	Undo,
	AddText(String),
	MoveCursor(i32, i32),
	Replace { from: String, to: String },
}

/**
 * pod: Enums support methods and associated functions
 */
impl Command {
	fn serialize(&self) -> String {
		serde_json::to_string(self).unwrap()
	}
}

pub fn enums_instance() {
	let book = Product {
		name: String::from("1984"),
		category: Category::Books,
	};
	println!("book: {:?}", book);
}

pub fn enums_variants() {
	let cmd1 = Command::Undo;
	let cmd2 = Command::AddText(String::from("Hello Ferris"));
	let cmd3 = Command::MoveCursor(10, 30);
	let cmd4 = Command::Replace {
		from: String::from("Hello"),
		to: String::from("Hi"),
	};
	println!("cmd1: {:?}", cmd1);
	println!("cmd2: {:?}", cmd2);
	println!("cmd3: {:?}", cmd3);
	println!("cmd4: {:?}", cmd4);
}

pub fn enums_implementation_blocks() {
	let cmd1 = Command::AddText(String::from("Hello Ferris"));
	let json1 = cmd1.serialize();
	println!("json1: {:?}", json1)
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn run_enums_instance() {
		enums_instance();
		assert!(true)
	}

	#[test]
	fn run_enums_variants() {
		enums_variants();
		assert!(true)
	}

	#[test]
	fn run_enums_implementation_blocks() {
		enums_implementation_blocks();
		assert!(true)
	}
}
