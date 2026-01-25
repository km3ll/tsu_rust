use std::{fs::File, io::Read};

/**
 * pod: Result Enum
 * - Ok(T)
 * - Err(E)
 * - unwrap()
 * - expect("msg")
 */

fn get_user(id: String) -> Result<u32, String> {
	if id.is_empty() {
		Err(String::from("ID cannot be empty"))
	} else {
		Ok(1)
	}
}

fn recoverable_errors_basic() {
	let r1: Result<u32, String> = get_user(String::from(""));
	println!("r1: {:?}", r1);
	let r2: Result<u32, String> = get_user(String::from("1100"));
	println!("r2: {:?}", r2);
}

fn recoverable_errors_file() {
	let r1: Result<File, std::io::Error> = File::open("example.txt");
	println!("r1: {:?}", r1);
}

fn recoverable_errors_match() {
	let r1: Result<File, std::io::Error> = File::open("example.txt");
	let file = match r1 {
		Ok(file) => file,
		Err(error) => {
			panic!("Failed to open file: {:?}", error);
		}
	};
}

fn recoverable_errors_unwrap() {
	let file: File = File::open("example.txt").unwrap();
}

fn recoverable_errors_expect() {
	let file: File = File::open("example.txt").expect("Failed to open file.");
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn run_recoverable_errors_basic() {
		recoverable_errors_basic();
	}

	#[test]
	fn run_recoverable_errors_file() {
		recoverable_errors_file();
	}

	#[test]
	#[should_panic]
	fn run_recoverable_errors_match() {
		recoverable_errors_match();
	}

	#[test]
	#[should_panic]
	fn run_recoverable_errors_unwrap() {
		recoverable_errors_unwrap();
	}

	#[test]
	#[should_panic]
	fn run_recoverable_errors_expect() {
		recoverable_errors_expect();
	}
}
