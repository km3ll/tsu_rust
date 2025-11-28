#[derive(Debug)]
struct User {
	active: bool,
	username: String,
	email: String,
	sign_in_count: u64,
}

fn create_user(username: String, email: String) -> User {
	let n1 = r#"
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

fn structs() {
	let n1 = r#"
	pod: Struct
	- Custom data type that packages together related values that make up a meaningful group
	- The names and types of its data are called fields
	- During instantiation we don't have to specify the fields in the same order of declaration
	---"#;
	println!("{n1}");
}

fn structs_usage() {
	let mut u1 = User {
		active: false,
		username: String::from("guest1"),
		email: String::from("guest1@example.com"),
		sign_in_count: 1,
	};
	println!("Structs: u1: {:?}", u1);

	let mut u2 = create_user(
		String::from("guest2"),
		String::from("guest2@example.com"),
	);
	println!("Structs: u2: {:?}", u2);
}


fn structs_other_instances() {
	// TODO
}

fn structs_() {

}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn run_structs() {
		structs();
	}

	#[test]
	fn run_structs_usage() {
		structs_usage();
	}

	#[test]
	fn run_structs_() {
		structs_();
	}
}
