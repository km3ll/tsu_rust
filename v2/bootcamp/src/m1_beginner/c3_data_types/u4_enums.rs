pub fn enums() {
	println!("----------");
	println!("Enums");

	println!("Product");
	let product = Product {
		name: String::from("1984"),
		category: Category::Books,
	};
	println!(" > name: {}", product.name);

	println!("Command");
	let cmd1 = Command::Undo;
	let cmd2 = Command::Redo;
	let cmd3 = Command::AddText(String::from("Ferris"));
	let cmd4 = Command::Replace {
		from: String::from("a"),
		to: String::from("b"),
	};
	let json = cmd1.serialize();
	println!(" > JSON: {}", json);
}

struct Product {
	name: String,
	category: Category,
}

enum Category {
	Books,
	Clothing,
	Electronics,
}

/**
 * Enum variants can have data associated with them
 * - We can add methods and associated functions using implementation blocks
 */
enum Command {
	Undo,
	Redo,
	AddText(String),
	MoveCursor(i32, i32),
	Replace { from: String, to: String },
}

impl Command {
	fn serialize(&self) -> String {
		String::from("{content}")
	}
}
