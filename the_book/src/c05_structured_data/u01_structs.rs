//! # Defining and Instantiating Structs

#[derive(Debug)]
struct User {
	active: bool,
	username: String,
	email: String,
	sign_in_count: u64,
}

#[derive(Debug)]
struct Color(i32, i32, i32);

#[derive(Debug)]
struct Point(i32, i32, i32);

#[derive(Debug)]
struct AlwaysEqual;

fn create_user(username: String, email: String) -> User {
	User {
		username,
		email,
		active: true,
		sign_in_count: 1,
	}
}

fn structs(username: String, email: String) -> User {
	let n1 = r#"
	---
	pod: Struct
	- Custom data type that packages together related values that make up a meaningful group
	- The names and types of its data are called `fields`
	- During instantiation we don't have to specify the fields in the same order of declaration

	pod: Field Init Shorthand Syntax
	- When a parameter and a field have the same name, we only need to write it once
	---"#;
	println!("{n1}");

	User {
		username,
		email,
		active: true,
		sign_in_count: 1,
	}
}

fn structs_usage() {
	println!("Structs");
	let mut u1 = User {
		active: false,
		username: String::from("guest1"),
		email: String::from("guest1@example.com"),
		sign_in_count: 1,
	};
	println!(" > u1: {:?}", u1);

	let mut u2 = create_user(String::from("guest2"), String::from("guest2@example.com"));
	println!(" > u2: {:?}", u2);
}

fn structs_update_syntax() {
	let n1 = r#"
	---
	pod: Struct Update Syntax
	- The base instance must come last after colons `..`
	- Uses equals `=` operator because it moves the data
	---"#;
	println!("{n1}");

	println!("Update syntax");
	let u1 = create_user(String::from("guest1"), String::from("changeme@example.com"));
	let u2 = User {
		username: String::from("guest2"),
		..u1
	};

	// Error: value used after being moved
	// println!("Structs updated u1: {u1:?}");
	println!(" > updated u2: {u2:?}");
}

fn structs_tuple() {
	let n1 = r#"
	---
	pod: Tuple Structs
	- Named structs with no names associated to their fields
	- Each tuple struct is its own type
	- Deconstructing them requires you to name the type of the struct
	---"#;
	println!("{n1}");

	println!("Tuple struct");
	let black = Color(0, 0, 0);
	println!(" > color: {black:?}");

	let origin = Point(5, 3, 9);
	println!(" > point: {origin:?}");

	let Point(x, y, z) = origin;
	println!(" > deconstructed x: {x}, y: {y}, z: {z}");
}

fn structs_unit_like() {
	let n1 = r#"
	---
	pod: Unit-Like Structs
	- Structs that don't have any fields that behave similar to the unit type ()
	- Useful when you need to implement a trait but don't have any data to store
	---"#;
	println!("{n1}");

	println!("Unit-like structs");
	let subject = AlwaysEqual;
	println!(" > subject: {subject:?}");
}

fn structs_ownership() {
	let n1 = r#"
	---
	pod: Struct Lifetimes
	- Ensure that data referenced by a struct is valid for as long as the struct is
	---"#;
	println!("{n1}");
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn run_structs_usage() {
		structs_usage();
	}

	#[test]
	fn run_structs_update_syntax() {
		structs_update_syntax();
	}

	#[test]
	fn run_structs_tuple() {
		structs_tuple();
	}

	#[test]
	fn run_structs_unit_like() {
		structs_unit_like();
	}

	#[test]
	fn run_structs_ownership() {
		structs_ownership();
	}
}
