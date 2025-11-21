pub fn slices_range_syntax() {
	let n1 = r#"
	pod: Slices
	- References to a contiguous sequence of elements in a collection
	- Used to reference part of a collection
	---"#;
	println!("{n1}");

	let tweet: String = String::from("This is my tweet and it's very long.");
	let s1: &str = &tweet[..20]; // beginning
	let s2: &str = &tweet[20..]; // ending
	let s3: &str = &tweet[..];   // entire
}

pub fn slices_string_types() {
	let n1 = r#"
	pod: String
	- Growable, heap allocated string (UTF-8 encoded)
	---"#;
	println!("{n1}");

	let tweet: String = String::from("This is my tweet and it's very long.");
	let t1: &str = &tweet[..20];
}

pub fn slices_string_literal() {
	let n1 = r#"
	pod: String slice '&str'
	- Immutable sequence of UTF-8 bytes somewhere in memory
	  (stack, heap, or static memory)
	- Handle behind a reference (&str) because length of
	  sequence is unknown at compile time
	- All string literals are string slices
	---"#;
	println!("{n1}");

	let s4: &str = "Ferris the crab";
}

pub fn slices_string_slice_and_functions() {
	fn trim_tweet(tweet: &String) -> &str {
		&tweet[..16]
	}
	let tweet: String = String::from("This is my tweet and it's very long.");
	let s5: &str = trim_tweet(&tweet);
}

pub fn slices_dref_coercion() {
	let n1 = r#"
	pod: DRef Coercion
	- Parameter type '&str' works for both: reference to string and string slice
	---"#;
	println!("{n1}");

	fn trim_tweet(tweet: &str) -> &str {
		&tweet[..16]
	}

	let tweet: String = String::from("This is my tweet and it's very long.");
	let s6: &str = trim_tweet(&tweet);
	let s7: &str = trim_tweet("This is my tweet and it's very long.");
}

pub fn slices_vector_slices() {
	let n1 = r#"
	pod: Debug Formatting
	- {:?} syntax
	---"#;
	println!("{n1}");

	let v1: [i32; 6] = [1, 2, 3, 4, 5, 6];
	let v2: &[i32] = &v1[..3];
	// println!("{:?}", v1);
	// println!("{:?}", v2);
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn run_slices_range_syntax() {
		slices_range_syntax();
	}

	#[test]
	fn run_slices_string_types() {
		slices_string_types();
	}

	#[test]
	fn run_slices_string_literal() {
		slices_string_literal();
	}

	#[test]
	fn run_slices_string_slice_and_functions() {
		slices_string_slice_and_functions();
	}

	#[test]
	fn run_slices_dref_cohersion() {
		slices_dref_coercion();
	}

	#[test]
	fn run_slices_vector_slices() {
		slices_vector_slices();
	}
}
