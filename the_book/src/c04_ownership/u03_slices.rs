//! # The Slice Type

fn first_word_v1(s: &String) -> usize {
	let n1 = r#"
	pod: Byte Literal Syntax
	- A notation that expresses a single byte (or sequence) in a clear, machine-aligned format
	- `b ` (space)
	---"#;
	println!("{n1}");

	let bytes = s.as_bytes();
	for (i, &item) in bytes.iter().enumerate() {
		if item == b' ' {
			return i;
		}
	}
	s.len()
}

fn first_word_v2(s: &String) -> &str {
	let bytes = s.as_bytes();
	for (i, &item) in bytes.iter().enumerate() {
		if item == b' ' {
			return &s[0..i];
		}
	}
	&s[..]
}

fn first_word_v3(s: &str) -> &str {
	let bytes = s.as_bytes();
	for (i, &item) in bytes.iter().enumerate() {
		if item == b' ' {
			return &s[0..i];
		}
	}
	&s[..]
}

fn slices() {
	let n1 = r#"
	pod: Slices
	- Reference a contiguous sequence of elements in a collection
	- Do not have ownership
	---
	pod: Idiomatic Rust
	- Functions do not take ownership of their arguments unless they need to
	---"#;
	println!("{n1}");
}

fn slices_string() {
	let n1 = r#"
	pod: String Slice
	- A reference to (portion of) a contiguous sequence of elements of a String
	- starting_index: first position in the slice
	_ ending_index: one more than the last position in the slice
	- With range syntax, you can drop the value before/after the two periods
	- You can drop both values of range syntax to take a slice of the entire string
	- Range indices must occur at valit UTF-8 character boundaries
	- The creation of a slice in the middle of a multibyte character will produce an error
	---"#;
	println!("{n1}");

	let s1 = String::from("Hello World");
	let s2 = &s1[0..5];
	let s3 = &s1[6..11];
	println!("Slices: s2: {s2}, s3: {s3}");

	let s4 = &s1[..4];
	let s5 = &s1[7..];
	println!("Slices: s4: {s4}, s5: {s5}");

	let s6 = &s1[0..s1.len()];
	let s7 = &s1[..];
	println!("Slices: s6: {s6}, s7: {s7}");
}

fn slices_first_word_v1() {
	let n1 = r#"
	pod: State Disconnection
	- index is not connected to the state of s1
	---"#;
	println!("{n1}");

	let mut s1 = String::from("Hello World");
	let index = first_word_v1(&s1);
	s1.clear();
	println!("Slices cleared s1: {s1}, index: {index}");
}

fn slices_first_word_v2() {
	let mut s1 = String::from("Hello World");
	// Error: cannot borrow `s1` as mutable because it is also borrowed as immutable
	// let index = first_word_v2(&s1);
	s1.clear()
}

fn slices_string_literals() {
	let n1 = r#"
	pod: String Literals as Slices
	- Literals are stored inside the binary
	- A `&str` (immutable reference) is a slice pointing to a specific point in the binary
	---"#;
	println!("{n1}");

	let s1 = "Hello, pod!";
	println!("Slices: immutable reference s1: {s1}");
}

fn slices_as_parameters() {
	let n1 = r#"
	pod: String Slices as Parameters
	- Function parameter `s: &str` allows to use the function on both &String and &str values
	- This flexibility takes advantages of Deref Coercion
	---"#;
	println!("{n1}");

	let my_string: String = String::from("Hello, Pod");

	let s2: &str = first_word_v3(&my_string[0..6]);
	println!("Slices: deref string: s2: {s2}");

	let s3: &str = first_word_v3(&my_string[..]);
	println!("Slices: deref string: s3: {s3}");

	let s4: &str = first_word_v3(&my_string);
	println!("Slices: deref string: s4: {s4}");

	let my_literal: &str = "Hello, Ferris!";

	let s5: &str = first_word_v3(&my_literal[0..6]);
	println!("Slices: deref literal: s5: {s5}");

	let s6: &str = first_word_v3(&my_literal[..]);
	println!("Slices: deref literal: s6: {s6}");

	let s7: &str = first_word_v3(my_literal);
	println!("Slices: deref literal: s7: {s7}");
}

fn slices_array() {
	let nums: [i8; 5] = [1, 2, 3, 4, 5];
	let slice: &[i8] = &nums[1..3];
	for num in slice {
		println!("Slices: vector: num: {num}");
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn run_slices() {
		slices();
	}

	#[test]
	fn run_slices_string() {
		slices_string();
	}

	#[test]
	fn run_slices_first_word_v1() {
		slices_first_word_v1();
	}

	#[test]
	fn run_slices_first_word_v2() {
		slices_first_word_v2();
	}

	#[test]
	fn run_slices_string_literals() {
		slices_string_literals();
	}

	#[test]
	fn run_slices_array() {
		slices_array();
	}
}
