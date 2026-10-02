//! # Slices

fn trim_tweet(tweet: &String) -> &str {
    println!("Slices: trimming tweet: {tweet}");
    &tweet[..16]
}

fn trim_tweet_deref(tweet: &str) -> &str {
    println!("Slices: trimming deref tweet: {tweet}");
    &tweet[..16]
}

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
    let s3: &str = &tweet[..]; // entire
    println!("Slices: s2: '{s2}', s1: '{s1}'");
}

pub fn slices_string_types() {
    let n1 = r#"
	pod: String
	- Growable, UTF-8 encoded, heap-allocated value
	---"#;
    println!("{n1}");

    let tweet: String = String::from("This is my tweet and it's very long.");
    let s1: &str = &tweet[..20];
    println!("Slices: s1: {s1}");
}

pub fn slices_string_literal() {
    let n1 = r#"
	pod: String Slice
	- Immutable sequence of UTF-8 bytes somewhere in memory (stack, heap, or static)
	- Handled behind a reference `&str` because length of sequence is unknown at compile time
	- All string literals are string slices
	---"#;
    println!("{n1}");

    let s4: &str = "Ferris the crab";
    println!("Slices, string literal: {s4}");
}

pub fn slices_string_slice_and_functions() {
    let tweet: String = String::from("This is my tweet and it's very long.");
    let s5: &str = trim_tweet(&tweet);
    println!("String slice from string reference: s5: {s5}");
}

pub fn slices_deref_coercion() {
    let n1 = r#"
	pod: Deref Coercion
	- An automatic conversion the compiler performs when a type implements the `Deref` trait
	- Allows a type to be implicitly treated as a reference to its target type
	- Parameter type `&str` works for both: reference to string and string slice
	- It exists to make smart-pointer types (Box, Arc, String) behave like normal references
	---"#;
    println!("{n1}");

    let tweet: String = String::from("This is my tweet and it's very long.");
    let s1: &str = "This is another tweet.";
    let s6: &str = trim_tweet_deref(&tweet);
    let s7: &str = trim_tweet_deref(s1);
    println!("Deref slices: s6: {s6}, s7: {s7}");
}

pub fn slices_vector_slices() {
    let n1 = r#"
	pod: Debug Formatting
	- The standardized, developer-focused way to print a value for inspection
	- A type supports it by implementing the std::fmt::Debug trait
	- `{:?}` syntax
	---"#;
    println!("{n1}");

    let v1: [i32; 6] = [1, 2, 3, 4, 5, 6];
    let v2: &[i32] = &v1[..3];
    println!("Debug format: v2: {:?}", v2);
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
    fn run_slices_deref_coercion() {
        slices_deref_coercion();
    }

    #[test]
    fn run_slices_vector_slices() {
        slices_vector_slices();
    }
}
