fn types_core() {
	let n1 = r#"
	pod: Data Types
	- Rust is a statically typed language (must know types at compile time)
	---
	pod: Scalar Types
	- Represent a single value. Rust has four:
	  - Integers
	  - Floating Point
	  - Booleans
	  - Characters
	---
	pod: Compound Types
	- Group multiple values into one type: tuples and arrays
	---
	pod: Panicking
	- Term used when a program exits with an error
	---"#;
	println!("{n1}");
}

fn types_scalar_integer() {
	let n1 = r#"
	pod: Integer
	- A number without a fractional component
	- Signed variants: i8, i16, i32, i64, i128, isize
	- Unsigned variants: u8, u16, u32, u64, u128, usize
	- Number literals can use underscore (_) as a visual separator
	---"#;
	println!("{n1}");

	let x: i16 = 1_000;
	println!("integer literal: x: {x}")
}


fn types_scalar_floating() {
	let n1 = r#"
	pod: Floating-Point Numbers
	- Numbers with decimal points (f32, f64)
	- All floating-point types are signed
	---"#;
	println!("{n1}");

	let x: f32 = 2.0;
	let y: f64 = 3.0;
	println!("floating-point numbers: x: {x}, y: {y}")
}

fn types_numeric_operations() {
	println!("operation");
	// addition
	let sum = 5 + 10;
	println!("> addition: {sum}");

	// subtraction
	let difference = 95.5 - 4.3;
	println!("> subtraction: {difference}");

	// multiplication
	let product = 4 * 30;
	println!("> multiplication: {product}");

	// division
	let quotient = 56.7 / 32.2;
	println!("> division: quotient: {quotient}");
	let truncated = -5 / 3;
	println!("> division: truncated: {truncated}");

	// remainder
	let remainder = 43 % 5;
	println!("> remainder: {remainder}")

}

fn types_scalar_boolean() {
	let b1: bool = true;
	println!("boolean: {b1}")
}

fn types_scalar_character() {
	let n1 = r#"
	pod: Type: char
	- Language's most primitive alphabetic type
	- Literals are specified with single quotes
	---"#;
	println!("{n1}");

	let c1: char = 'f';
	let c2: char = '🦀';
	println!("char: ferris: {c2}")
}

fn types_compound_tuple() {
	let n1 = r#"
	pod: Tuple
	- Groups values with a variety of types
	- Types can be different
	- Have fixed length
	- Can be destructured using pattern matching
	- Elements can be accessed by using a period (.)
	- The first index is Zero (0)
	---
	pod: Destructuring
	- Breaking a tuple into its parts
	---
	pod: Unit
	- A tuple without any values ()
	- Represents an empty value or an empty return type
	---"#;
	println!("{n1}");

	let tup1: (i32, f64, i8) = (5, 6.4, -3);
	println!("tuple: {:?}", tup1);

	let (x, y, z): (i32, f64, i8) = tup1;
	println!("tuple (destructured): y: {y}");
}

fn types_compound_array() {
	let n1 = r#"
	pod: Array
	- Every element must have the same type
	- Its data is allocated on the stack
	- Have a fixed length
	- The first index is Zero (0)
	- Panics when attempting to access an invalid index
	---"#;
	println!("{n1}");

	let a1 = [1, 2, 3, 4, 5];
	println!("array: {:?}", a1);

	let days: [&str; 7] = ["Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday", "Sunday"];
	println!("array: {:?}", days);
	println!("array: first: {}", days[0]);

	let switches = [false; 5];
	println!("array (repeated): {:?}", switches);
}

fn types_compound_vector() {
	let n1 = r#"
	pod: Vector
	- It is allowed to grow or shrink in size
	- Its data is allocated on the heap
	---"#;
	println!("{n1}");

	let v1: Vec<String> = vec![
		String::from("One"),
		String::from("Two"),
		String::from("Three"),
	];
	println!("vector: {:?}", v1)
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn run_types_core() {
		types_core();
	}

	#[test]
	fn run_types_scalar_integer() {
		types_scalar_integer();
	}

	#[test]
	fn run_types_scalar_floating() {
		types_scalar_floating();
	}

	#[test]
	fn run_types_numeric_operations() {
		types_numeric_operations();
	}

	#[test]
	fn run_types_scalar_boolean() {
		types_scalar_boolean();
	}

	#[test]
	fn run_types_scalar_character() {
		types_scalar_character();
	}

	#[test]
	fn run_types_compound_tuple() {
		types_compound_tuple();
	}

	#[test]
	fn run_types_compound_array() {
		types_compound_array();
	}

	#[test]
	fn run_types_compound_vector() {
		types_compound_vector();
	}
}
