//! # Enums

use serde::Serialize;

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

#[derive(Debug, Serialize)]
enum Command {
    Undo,
    AddText(String),
    MoveCursor(i32, i32),
    Replace { from: String, to: String },
}

impl Command {
    fn serialize(&self) -> String {
        println!(" > impl: serialize(&self)");
        serde_json::to_string(self).unwrap()
    }
}

pub fn enums_instance() {
    let n1 = r#"
    ---
	pod: Enums
	- Allow to define a type enumerating its variance
	- Variants can have data associated with them
	- Support methods and associated functions
	---"#;
    println!("{n1}");

    println!("Enums");
    let p1 = Product {
        name: String::from("1984"),
        category: Category::Books,
    };
    println!(" > p1: {p1:?}");
}

pub fn enums_variants() {
    println!("Enum Variants");
    let c1 = Command::Undo;
    let c2 = Command::AddText(String::from("Hello Ferris"));
    let c3 = Command::MoveCursor(10, 30);
    let c4 = Command::Replace {
        from: String::from("Hello"),
        to: String::from("Hi"),
    };
    println!(" > commands:");
    println!(" > c1: {c1:?}");
    println!(" > c2: {c2:?}");
    println!(" > c3: {c3:?}");
    println!(" > c4: {c4:?}");
}

pub fn enums_implementation_blocks() {
    println!("Enums");
    let cmd = Command::AddText(String::from("Hello Ferris"));
    println!(" > main: cmd: {cmd:?}");

    let json = cmd.serialize();
    println!(" > main: json: {json:?}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_enums_instance() {
        enums_instance();
    }

    #[test]
    fn run_enums_variants() {
        enums_variants();
    }

    #[test]
    fn run_enums_implementation_blocks() {
        enums_implementation_blocks();
    }
}
