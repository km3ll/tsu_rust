use std::fs::{self, File};
use std::io::{self, ErrorKind, Read};

fn read_username_v1() -> Result<String, io::Error> {
	let res1 = File::open("hello.txt");

	let mut file = match res1 {
		Ok(file) => file,
		Err(err) => return Err(err),
	};

	let mut username = String::new();
	match file.read_to_string(&mut username) {
		Ok(_) => Ok(username),
		Err(err) => Err(err),
	}
}

fn read_username_v2() -> Result<String, io::Error> {
	let mut file: File = File::open("hello.txt")?;
	let mut username: String = String::new();
	file.read_to_string(&mut username)?;
	Ok(username)
}

fn read_username_v3() -> Result<String, io::Error> {
	let mut username = String::new();
	File::open("hello.txt")?.read_to_string(&mut username)?;
	Ok(username)
}

fn read_username_v4() -> Result<String, io::Error> {
	fs::read_to_string("hello.txt")
}

fn read_last_character_v5(text: &str) -> Option<char> {
	text.lines().next()?.chars().last()
}

fn recoverable_definition() {
	let n1 = r#"
	pod: Result
	- Loaded into scope by the prelude
	- 'unwrap' returns the value inside the Ok variant or calls the panic! macro upon the Err variant
	- 'expect' lets choose the error message for the panic! macro
	- Most Rustaceans choose 'expect' rather than 'unwrap' and give more context about why the operation is expected to succeed
	---
	pod: ErrorKind
	- This enum has variants representing kinds of errors that might result from an io operation
	---"#;
	println!("{n1}");

	let path = "hello.txt";
	match File::open(path) {
		Ok(file) => println!("Recoverable: file: {file:?}"),
		Err(err) => panic!("Recoverable: problem opening path: {path}, error: {err:?}"),
	}
}

fn recoverable_error_kind() {
	let path = "hello.txt";
	match File::open(path) {
		Ok(file) => println!("Recoverable: file: {file:?}"),
		Err(err) => match err.kind() {
			ErrorKind::NotFound => match File::create(path) {
				Ok(fc) => println!("Recoverable: file created: {fc:?}"),
				Err(err) => println!("Recoverable: error creating file: {err:?}"),
			},
			_ => panic!("Recoverable: problem opening path: {path}, error: {err:?}"),
		},
	}
}

fn recoverable_unwrap() {
	println!("Recoverable: unwrap()");
	let path = "hello.txt";
	File::open(path).unwrap();
}

fn recoverable_expect() {
	println!("Recoverable: expect()");
	let path = "hello.txt";
	File::open(path).expect(&format!("Recoverable: file not found: {path}"));
}

fn recoverable_propagating_v1() {
	let n1 = r#"
	pod: Propagating Errors
	- Returning the error to the calling code instead of handling it
	---"#;
	println!("{n1}");

	let res = read_username_v1();
	println!("Recoverable: propagated v1: {res:?}");
}

fn recoverable_propagating_v2() {
	let n1 = r#"
	pod: Operator: question mark (?)
	- Placed after a Result value
	- Upon an Ok, the value inside will be returned from this expression
	- Upon an Err, the err will be returned from the whole function, as if we had used the return keyword
	- When ? is called on an error
	  - it goes through the 'from' function of the 'From' trait
	  - the error type is converted into the error defined in the return type of the function
	  - then only one error type represents all the ways a function might fail
	- Can be used in functions whose return type is compatible with the value the ? is used on
	- Can be used with Result, Option or another type that implements FromResidual
	---"#;
	println!("{n1}");

	let res = read_username_v2();
	println!("Recoverable: propagated v2: {res:?}");
}

fn recoverable_propagating_v3() {
	let res = read_username_v3();
	println!("Recoverable: propagated v3: {res:?}");
}

fn recoverable_propagating_v4() {
	let res = read_username_v4();
	println!("Recoverable: propagated v4: {res:?}");
}

fn recoverable_option() {
	let op1 = read_last_character_v5("");
	println!("Recoverable: propagated option1: {op1:?}");
	let op2 = read_last_character_v5("Hello");
	println!("Recoverable: propagated option1: {op2:?}");
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	#[should_panic]
	fn run_recoverable_definition() {
		recoverable_definition();
	}

	#[test]
	#[should_panic]
	fn run_recoverable_unwrap() {
		recoverable_unwrap();
	}

	#[test]
	#[should_panic]
	fn run_recoverable_expect() {
		recoverable_expect();
	}

	#[test]
	fn run_recoverable_propagating_v1() {
		recoverable_propagating_v1();
	}

	#[test]
	fn run_recoverable_propagating_v2() {
		recoverable_propagating_v2();
	}

	#[test]
	fn run_recoverable_propagating_v3() {
		recoverable_propagating_v3();
	}

	#[test]
	fn run_recoverable_propagating_v4() {
		recoverable_propagating_v4();
	}

	#[test]
	fn run_recoverable_option() {
		recoverable_option();
	}
}
