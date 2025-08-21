use std::fmt::format;

pub fn strings() {
	println!("----------");
	println!("Strings");

	println!("UTF-8 Encoded");
	let s1: &str = "Hello Ferris 🦀";
	println!("s1: {s1}");

	println!("Creating");
	let s2: String = String::from("Hello 🦀");
	println!("s2: {s2}");

	let s3: String = "Ferris 🦀".to_string();
	println!("s3: {s3}");

	let s4: String = "🦀 Hello".to_owned();
	println!("s4: {s4}");

	let s5: &str = &s4[..];
	println!("s5: {s5}");

	println!("Manipulating");
	println!("push_str");
	let mut s6: String = String::from("Hello ");
	s6.push_str("Ferris ");
	s6.push_str("🦀");
	println!("s6: {s6}");

	println!("replace_range");
	s6.replace_range(.., "Hello Pod!");
	println!("s6: {s6}");

	println!("concatenation +");
	let s7: String = String::from("Hello ");
	let s8: String = String::from("Ferris");
	let s9 = s7 + &s8;
	println!("s9: {s9}");

	/**
	 * Format macro
	 * - Less efficient because it copies the contents of strings
	 * - Can take string and string slices as parameters
	 */
	println!("format macro");
	let s11: String = String::from("tic");
	let s12: String = String::from("tac");
	let s13: &str = "toe";
	let s14: String = format!("{}, {}, {}", s11, s12, s13);
	println!("s14: {s14}");

	println!("concat: vector");
	let s15: String = ["Hello ", "Motto"].concat();
	println!("s15: {s15}");

	println!("concat: format!");
	let s16: String = format!("{}{}", "Hello ", "Ferris");
	println!("s16: {s16}");

	println!("concat: slice");
	let s17: &str = concat!("Hello ", "Pod");
	println!("s17: {s17}");

	println!("concat: string + slice");
	let s18: String = String::from("Hello ");
	let s19: String = s18 + "there"; // String type must be first
	println!("s19: {s19}");

	/**
	 * Extracting from string
	 * - UTF-8 characters could be between 1 to 4 bytes
	 */
	println!("Extracting");
	let s20: &str = "🦀🦀🦀🦀🦀";
	let s21: &str = &s20[..4];
	println!("s21: {s21}");

	/**
	 * Iterating
	 */
	println!("Iterating");

	println!("bytes");
	for b1 in "Hello 🦀".bytes() {
		println!("{}", b1);
	}

	println!("characters");
	for c1 in "Hello 🦀".chars() {
		println!("{}", c1);
	}
}
