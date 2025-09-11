use unicode_segmentation::UnicodeSegmentation;

pub fn strings_acronyms() {
	/**
	 * pod: ASCII. American Standard Code for Information Interchange
	 * An encoding which maps integers to characters.
	 */
	println!("ASCII. American Standard Code for Information Interchange");

	/**
	 * pod: UTF-8. Unicode Transformation Format
	 * (8 bits = 1 byte)  
	 */
	println!("UTF-8. Unicode Transformation Format");
}

pub fn strings_utf_8() {
	println!("UTF-8 Encoded");
	let s1: &str = "Hello Ferris 🦀";
	println!(" s1: {s1}\n");

	println!("String::from");
	let s2: String = String::from("Hello 🦀");
	println!(" s2: {s2}\n");

	println!("to_string");
	let s3: String = "Ferris 🦀".to_string();
	println!(" s3: {s3}\n");

	println!("to_owned");
	let s4: String = "🦀 Hello".to_owned();
	println!(" s4: {s4}\n");

	println!("slice");
	let s5: &str = &s4[..];
	println!(" s5: {s5}\n");

	println!("push_str");
	let mut s6: String = String::from("Hello ");
	s6.push_str("Ferris ");
	s6.push_str("🦀");
	println!(" s6: {s6}\n");

	println!("replace_range");
	s6.replace_range(.., "Hello Pod!");
	println!(" s6: {s6}\n");
}

pub fn strings_slice() {
	/**
	 * pod: String slice type (&str)
	 * - is a view of of a sequence of UTF-8 enconded bytes.
	 * - does not own the underlying data
	 */
	println!("String slice type (&str)");
}

pub fn strings_string() {
	/**
	 * pod: String type
	 * - is a growable, mutable, owned UTF-8 encoded string
	 * - always allocated on the Heap
	 */
	println!("String type");
}

pub fn strings_concatenation() {
	println!("plus operator (+)");
	let s7: String = String::from("Hello ");
	let s8: String = String::from("Ferris");
	let s9 = s7 + &s8;
	println!(" s9: {s9}\n");

	/**
	 * pod: Format macro
	 * - Less efficient because it copies the contents of strings
	 * - Can take string and string slices as parameters
	 */
	println!("format!");
	let s11: String = String::from("tic");
	let s12: String = String::from("tac");
	let s13: &str = "toe";
	let s14: String = format!("{}, {}, {}", s11, s12, s13);
	println!(" s14: {s14}\n");

	/**
	 * pod: Concatenating a Vector
	 */
	println!("vector.concat");
	let s15: String = ["Hello ", "Motto"].concat();
	println!(" s15: {s15}\n");

	/**
	 * pod: Concat macro
	 */
	println!("concat!: slice");
	let s17: &str = concat!("Hello ", "Pod");
	println!(" s17: {s17}\n");

	println!("concat!: string + slice");
	let s18: String = String::from("Hello ");
	let s19: String = s18 + "there"; // String type must be first
	println!(" s19: {s19}\n");
}

pub fn strings_extracting() {
	/**
	 * pod: Extracting from string
	 * - UTF-8 characters could be between 1 to 4 bytes
	 */
	println!("Extracting");
	let s20: &str = "🦀🦀🦀🦀🦀";
	let s21: &str = &s20[..4];
	println!(" s20: {s20}");
	println!(" s21: {s21}");
}

pub fn strings_different_bytes() {
	/**
	 * pod: Different byte lenght
	 * - 🦀 is 8-byte long
	 * - Letters 'the' are 1-byte long
	 */
	println!("Different bytes");
	let s22: &str = "🦀the🦀";
	println!(" s22: {s22}");
}

pub fn strings_iterating() {
	/**
	 * pod: Finding a character takes linear time
	 * - we have to iterate over each character
	 */
	println!("Iterating over bytes .bytes()");
	for b1 in "Hello 🦀".bytes() {
		println!(" {}", b1);
	}
}

pub fn strings_graphemes() {
	/**
	 * pod: Graphemes are user-perceived characters
	 */
	println!("Graphemes .chars()");
	for c1 in "Hello 🦀".chars() {
		println!(" {}", c1);
	}
	println!("Graphemes .graphemes()");
	for c1 in "Hello 🦀".graphemes(true) {
		println!(" {}", c1);
	}
}

pub fn strings_and_functions() {
	fn format_it(p1: &str) -> String {
		return format!("[{}]", p1);
	}

	let s1 = "Hello Ferris";
	let s2 = String::from("Hello 🦀");

	println!("Deref cohersion");
	let s3: String = format_it(s1);
	let s4: String = format_it(&s2);
	println!(" s3: {}", s3);
	println!(" s4: {}", s4);
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn run_strings_acronyms() {
		strings_acronyms();
		assert!(true)
	}

	#[test]
	fn run_strings_utf_8() {
		strings_utf_8();
		assert!(true)
	}

	#[test]
	fn run_strings_slice() {
		strings_slice();
		assert!(true)
	}

	#[test]
	fn run_strings_string() {
		strings_string();
		assert!(true)
	}

	#[test]
	fn run_strings_concatenation() {
		strings_concatenation();
		assert!(true)
	}

	#[test]
	fn run_strings_extracting() {
		strings_extracting();
		assert!(true)
	}

	#[test]
	fn run_different_bytes() {
		strings_different_bytes();
		assert!(true)
	}

	#[test]
	fn run_strings_iterating() {
		strings_iterating();
		assert!(true)
	}

	#[test]
	fn run_strings_graphemes() {
		strings_graphemes();
		assert!(true)
	}

	#[test]
	fn run_strings_and_functions() {
		strings_and_functions();
		assert!(true)
	}
}
