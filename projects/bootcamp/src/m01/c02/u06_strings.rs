use unicode_segmentation::UnicodeSegmentation;

pub fn strings_acronyms() {
	let n1 = r#"
	pod: ASCII
	- American Standard Code for Information Interchange
	- An encoding which maps integers to characters.
	---"#;
	println!("{n1}");

	let n2 = r#"
	pod: UTF-8. Unicode Transformation Format
	- (8 bits = 1 byte)
	---"#;
	println!("{n2}");
}

pub fn strings_utf_8() {
	let s1: &str = "Hello Ferris 🦀";
	// println!("UTF-8 Encoded");
	// println!(" s1: {s1}\n");

	let s2: String = String::from("Hello 🦀");
	// println!("String::from");
	// println!(" s2: {s2}\n");

	let s3: String = "Ferris 🦀".to_string();
	// println!("to_string");
	// println!(" s3: {s3}\n");

	let s4: String = "🦀 Hello".to_owned();
	// println!("to_owned");
	// println!(" s4: {s4}\n");

	let s5: &str = &s4[..];
	// println!("slice");
	// println!(" s5: {s5}\n");

	let mut s6: String = String::from("Hello ");
	s6.push_str("Ferris ");
	s6.push_str("🦀");
	// println!("push_str");
	// println!(" s6: {s6}\n");

	s6.replace_range(.., "Hello Pod!");
	// println!("replace_range");
	// println!(" s6: {s6}\n");
}

pub fn strings_slice() {
	let n1 = r#"
	pod: String Slice Type (&str)
	- Is a view of a sequence of UTF-8 encoded bytes
	- Does not own the underlying data
	---"#;
	println!("{n1}");
}

pub fn strings_string() {
	let n1 = r#"
	pod: String Type
	- Is a growable, mutable, owned UTF-8 encoded string
	- Always allocated on the Heap
	---"#;
	println!("{n1}");
}

pub fn strings_concatenation() {
	let s7: String = String::from("Hello ");
	let s8: String = String::from("Ferris");
	let s9 = s7 + &s8;
	// println!("plus operator (+)");
	// println!(" s9: {s9}\n");

	let n1 = r#"
	pod: Macro: format!()
	- Less efficient because it copies the contents of strings
	- Can take string and string slices as parameters
	---"#;
	println!("{n1}");

	let s11: String = String::from("tic");
	let s12: String = String::from("tac");
	let s13: &str = "toe";
	let s14: String = format!("{}, {}, {}", s11, s12, s13);
	// println!("format!");
	// println!(" s14: {s14}\n");

	let n2 = r#"
	pod: Concatenating a Vector
	---"#;
	println!("{n2}");

	let s15: String = ["Hello ", "Motto"].concat();
	// println!("vector.concat");
	// println!(" s15: {s15}\n");

	let n3 = r#"
	pod: Macro: concat!()
	---"#;
	println!("{n3}");

	let s17: &str = concat!("Hello ", "Pod");
	// println!("concat!: slice");
	// println!(" s17: {s17}\n");

	let s18: String = String::from("Hello ");
	let s19: String = s18 + "there"; // String type must be first
	                              // println!("concat!: string + slice");
	                              // println!(" s19: {s19}\n");
}

pub fn strings_extracting() {
	let n1 = r#"
	pod: Extracting from String
	- UTF-8 characters could be between 1 and 4 bytes
	---"#;
	println!("{n1}");

	let s20: &str = "🦀🦀🦀🦀🦀";
	let s21: &str = &s20[..4];
	// println!("Extracting");
	// println!(" s20: {s20}");
	// println!(" s21: {s21}");
}

pub fn strings_different_bytes() {
	let n1 = r#"
	pod: Different Byte Length
	- 🦀 is 8-byte long
	- Letters 'the' are 1-byte long
	---"#;
	println!("{n1}");

	let s22: &str = "🦀the🦀";
	// println!("Different bytes");
	// println!(" s22: {s22}");
}

pub fn strings_iterating() {
	let n1 = r#"
	pod: Linear Time
	- Finding a character takes linear time.
	  We have to iterate over each character
	---"#;
	println!("{n1}");

	// println!("Iterating over bytes .bytes()");
	for b1 in "Hello 🦀".bytes() {
		// println!(" {}", b1);
	}
}

pub fn strings_graphemes() {
	let n1 = r#"
	pod: Graphemes
	- User-perceived characters
	---"#;
	println!("{n1}");

	// println!("Graphemes .chars()");
	for c1 in "Hello 🦀".chars() {
		// println!(" {}", c1);
	}

	// println!("Graphemes .graphemes()");
	for c1 in "Hello 🦀".graphemes(true) {
		// println!(" {}", c1);
	}
}

pub fn strings_and_functions() {
	fn format_it(p1: &str) -> String {
		return format!("[{}]", p1);
	}

	let s1 = "Hello Ferris";
	let s2 = String::from("Hello 🦀");

	let s3: String = format_it(s1);
	let s4: String = format_it(&s2);
	// println!("Deref coercion");
	// println!(" s3: {}", s3);
	// println!(" s4: {}", s4);
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn run_strings_acronyms() {
		strings_acronyms();
	}

	#[test]
	fn run_strings_utf_8() {
		strings_utf_8();
	}

	#[test]
	fn run_strings_slice() {
		strings_slice();
	}

	#[test]
	fn run_strings_string() {
		strings_string();
	}

	#[test]
	fn run_strings_concatenation() {
		strings_concatenation();
	}

	#[test]
	fn run_strings_extracting() {
		strings_extracting();
	}

	#[test]
	fn run_different_bytes() {
		strings_different_bytes();
	}

	#[test]
	fn run_strings_iterating() {
		strings_iterating();
	}

	#[test]
	fn run_strings_graphemes() {
		strings_graphemes();
	}

	#[test]
	fn run_strings_and_functions() {
		strings_and_functions();
	}
}
