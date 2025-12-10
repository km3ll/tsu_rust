fn strings_definition() {
	let n1 = r#"
	pod: Strings
	- 'str' (string slice) is the only string type in the core language
	- 'String' (provided in standard library) is a growable, mutable, owned, UTF-8 encoded string type
	- Implemented as a wrapper around a vector of bytes
	---"#;
	println!("{n1}");
}

fn strings_creation() {
	let n1 = r#"
	pod: Trait: Display
	- 'to_string' method is available on any type that implements the Display trait
	---"#;
	println!("{n1}");

	let mut s1 = String::new();
	println!("Strings: s1: {s1}");

	let slice = "Hello";
	let s2 = slice.to_string();
	println!("Strings: to_string s2: {s2}");

	let s3 = "Ferris".to_string();
	println!("Strings: to_string s3: {s3}");

	let s4 = String::from("Hello, pod!");
	println!("Strings: to_string s4: {s4}");
}

fn strings_utf8() {
	let vec1 = vec![
		String::from("Dobrý den"),
		String::from("Hello"),
		String::from("नमस्ते"),
		String::from("こんにちは"),
		String::from("안녕하세요"),
		String::from("你好"),
		String::from("Olá"),
		String::from("Здравствуйте"),
		String::from("Hola"),
	];
	println!("Strings: UTF-8");
	for hello in &vec1 {
		println!("> {hello}");
	}
}

fn strings_update() {
	let n1 = r#"
	pod: Strings: push_str()
	- Does not take ownership of the parameter (&str)
	---
	pod: Strings: push()
	- Adds a single character to the String
	---"#;
	println!("{n1}");

	let mut s1 = String::from("Hello");
	s1.push_str(", pod!");
	println!("Strings: push_str s1: {s1}");

	let mut s2 = String::from("Hello, pod");
	s2.push('!');
	println!("Strings: push s2: {s2}");
}

fn strings_concatenation() {
	let n1 = r#"
	pod: The + Operator
	- Takes ownership of first parameter (self doesn't have an &)
	- Appends a copy of the contents of s2
	- Returns ownership of the result
	---"#;
	println!("{n1}");

	let s1 = String::from("Hello");
	let s2 = String::from(", pod!");
	let s3 = s1 + &s2;
	println!("Strings: + s3: {s3}");

	let n1 = r#"
	pod: Macro: format!
	- Combines strings in more complicated ways
	- Does not take ownership of any of its parameters
	---"#;
	println!("{n1}");

	let s4 = String::from("tic");
	let s5 = String::from("tac");
	let s6 = String::from("toe");
	let s7 = format!("{s4}-{s5}-{s6}");
	print!("Strings: format s7: {s7}");
}

fn strings_indexing() {
	let n1 = r#"
	pod: Strings: indexing
	- Strings don't support indexing
	- String is wrapper over a Vec<u8>
	- Each Unicode scalar value takes 2 bytes of storage
	- An index into the string's bytes would not always correlate to a valid Unicode scalar value
	- Indexing operations are expected to take constant time, but Rust would have to walk through from beginning to end of strings
	---"#;
	println!("{n1}");

	let s1 = String::from("Hola");
	let len1 = s1.len();
	println!("String: 'hola' length: {len1}");

	let s2 = String::from("Здравствуйте");
	let len2 = s2.len();
	println!("String: 'Здравствуйте' length: {len2}");
}

fn strings_in_rust() {
	let n1 = r#"
	pod: Strings in Rust
	- As (1) bytes, (2) scalar values and (3) grapheme clusters (letters)
	---"#;
	println!("{n1}");

	let s1 = String::from("नमस्ते");
	println!("Strings: word as string: {s1}");

	let vec1: Vec<u8> = vec![
		224, 164, 168, 224, 164, 174, 224, 164, 184, 224, 165, 141, 224, 164, 164, 224, 165, 135,
	];
	println!("Strings: word as vector: {vec1:?}");

	let vec2: Vec<char> = vec!['न', 'म', 'स', '्', 'त', 'े'];
	println!("Strings: word as scalar: {vec2:?}");

	// let vec3: Vec<char> = vec!["न", "म", "स्", "ते"];
	// println!("Strings: word as graphemes: {vec3:?}");
}

fn strings_slicing() {
	let n1 = r#"
	pod: Strings: slices
	- You can use [] with a range to create a slice containing particular bytes
	---"#;
	println!("{n1}");

	let s1 = "Здравствуйте";
	let s2 = &s1[0..4];

	println!("Strings: slice with range: s1: {s1}, s2: {s2}");
}

fn strings_iterating() {
	let n1 = r#"
	pod: Strings: iterating chars or bytes
	- 'char' returns individual Unicode scalar values
	---"#;
	println!("{n1}");

	let s1 = "Здрав";
	println!("Strings: chars of s1: {s1}");
	for c1 in s1.chars() {
		println!("> {c1}");
	}

	println!("Strings: bytes of s1: {s1}");
	for b1 in s1.bytes() {
		println!("> {b1}");
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn run_strings_definition() {
		strings_definition();
	}

	#[test]
	fn run_strings_creation() {
		strings_creation();
	}

	#[test]
	fn run_strings_utf8() {
		strings_utf8();
	}

	#[test]
	fn run_strings_update() {
		strings_update();
	}

	#[test]
	fn run_strings_concatenation() {
		strings_concatenation();
	}

	#[test]
	fn run_strings_indexing() {
		strings_indexing();
	}

	#[test]
	fn run_strings_in_rust() {
		strings_in_rust();
	}

	#[test]
	fn run_strings_slicing() {
		strings_slicing();
	}

	#[test]
	fn run_strings_iterating() {
		strings_iterating();
	}
}
