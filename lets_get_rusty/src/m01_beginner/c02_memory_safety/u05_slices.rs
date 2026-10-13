//! # Slices

fn trim_tweet(p1: &String) -> &str {
    println!(" > func: trimming &String");
    &p1[..16]
}

fn trim_tweet_deref(p1: &str) -> &str {
    println!(" > func: trimming &str (deref)");
    &p1[..16]
}

pub fn slices_range_syntax() {
    let n1 = r#"
    ---
	pod: Slices
	- References to a contiguous sequence of elements in a collection
	- Used to reference part of a collection
	---"#;
    println!("{n1}");

    println!("Slices");

    let tweet: String = String::from("This is my tweet and it's very long.");
    println!(" > tweet: {tweet}");

    let s1: &str = &tweet[..20]; // beginning
    println!(" > beginning s1: {s1}");

    let s2: &str = &tweet[20..]; // ending
    println!(" > ending s2: {s2}");

    let s3: &str = &tweet[..]; // entire
    println!(" > entire s3: {s3}");
}

pub fn slices_string_types() {
    let n1 = r#"
    ---
	pod: String
	- Growable, UTF-8 encoded, heap-allocated value
	---"#;
    println!("{n1}");

    println!("Slices");

    let tweet: String = String::from("This is my tweet and it's very long.");
    println!(" > String: tweet: {tweet}");

    let s1: &str = &tweet[..];
    println!(" > Slice [..] s1: {s1}");
}

pub fn slices_string_literal() {
    let n1 = r#"
    ---
	pod: String Slice
	- Immutable sequence of UTF-8 bytes somewhere in memory (stack, heap, or static)
	- Handled behind a reference `&str` because length of sequence is unknown at compile time
	- All string literals are string slices
	---"#;
    println!("{n1}");

    println!("Slices");

    let s4: &str = "Ferris the crab";
    println!(" > String literal &str s4: {s4}");
}

pub fn slices_string_slice_and_functions() {
    println!("Slices");

    let tweet: String = String::from("This is my tweet and it's very long.");
    println!(" > main: tweet: {tweet}");

    let s5: &str = trim_tweet(&tweet);
    println!(" > main: trimmed &str s5: {s5}");
}

pub fn slices_deref_coercion() {
    let n1 = r#"
    ---
	pod: Deref Coercion
	- An automatic conversion the compiler performs when a type implements the `Deref` trait
	- Allows a type to be implicitly treated as a reference to its target type
	- Parameter type `&str` works for both: reference to string and string slice
	- It exists to make smart-pointer types (Box, Arc, String) behave like normal references
	---"#;
    println!("{n1}");

    println!("Slices");

    let s1: String = String::from("This is my tweet and it's very long.");
    println!(" > main: String s1: {s1}");

    let t1: &str = trim_tweet_deref(&s1);
    println!(" > main: String t1: {t1}");

    let s2: &str = "This is another tweet.";
    println!(" > main: slice &str s2: {s2}");

    let t2: &str = trim_tweet_deref(s2);
    println!(" > main: slice &str t2: {t2}");
}

pub fn slices_vector_slices() {
    let n1 = r#"
    ---
	trait: `Debug`
	- The standardized, developer-focused way to print a value for inspection
	- A type supports it by implementing the `std::fmt::Debug` trait
	- When used with the alternate format specifier `#?`, the output is pretty-printed
	- Sintax: `{:?}`, `{:#?}`, `{name:?}`
	---"#;
    println!("{n1}");

    println!("Slices");

    let v1: [i32; 6] = [1, 2, 3, 4, 5, 6];
    println!(" > format-string v1: {v1:?}");

    let v2: &[i32] = &v1[..3];
    println!(" > pretty-print v2: {v2:#?}");
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
