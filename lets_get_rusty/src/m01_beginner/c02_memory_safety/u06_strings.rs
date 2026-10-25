//! # Strings in Rust

use unicode_segmentation::UnicodeSegmentation;

fn format_slice(p1: &str) -> String {
    println!(" > func: format! [&str]");
    format!("[{p1}]")
}

pub fn strings_acronyms() {
    let n1 = r#"
    ---
	pod: ASCII
	- American Standard Code for Information Interchange
	- An encoding which maps integers to characters

	pod: UTF-8
	- Unicode Transformation Format (8 bits = 1 byte)
	---"#;
    println!("{n1}");
}

pub fn strings_utf_8() {
    println!("Strings");

    let s1: &str = "Hello Ferris 🦀";
    println!(" > &str s1: {s1}");

    let s2: String = String::from("Hello 🦀");
    println!(" > String: from s2: {s2}");

    let s3: String = "Ferris 🦀".to_string();
    println!(" > String: to_string s3: {s3}");

    let s4: String = "🦀 Hello".to_owned();
    println!(" > String: to_owned s4: {s4}");

    let s5: &str = &s4[..];
    println!(" > &str s5: {s5}");

    let mut s6: String = String::from("Hello ");
    println!(" > String: mut s6: {s6}");

    s6.push_str("Ferris ");
    s6.push_str("🦀");
    println!(" > String: mut s6: {s6}");

    s6.replace_range(.., "Hello Pod!");
    println!(" > String: replace_range mut s6: {s6}");
}

pub fn strings_slice() {
    let n1 = r#"
    ---
	pod: String Slice
	- Is a borrowed view of a sequence of UTF-8 encoded bytes
	- Does not own the underlying data, just references part (or all) of a String
	---"#;
    println!("{n1}");
}

pub fn strings_string() {
    let n1 = r#"
    ---
	pod: String Type
	- Is a owned, growable, heap-allocated UTF-8 text type
	---"#;
    println!("{n1}");
}

pub fn strings_concatenation() {
    let n1 = r#"
    ---
	macro: `format!()`
	- Creates a String using interpolation and formatting
	- It just returns the formatted text as an owned String
	- Less efficient because it copies the contents of strings
	- Can take string and string slices as parameters
	---"#;
    println!("{n1}");

    println!("String Concatenation");

    let s7: String = String::from("Hello ");
    println!(" > s7: {s7}");

    let s8: String = String::from("Ferris");
    println!(" > s8: {s8}");

    let s9 = s7 + &s8;
    println!(" > s7 + &s8 = s9: {s9}");

    println!("Macro: format!");
    let s11: String = String::from("tic");
    let s12: String = String::from("tac");
    let s13: &str = "toe";
    let s14: String = format!("{}, {}, {}", s11, s12, s13);
    println!(" > formatted owned s14: {}", s14);

    let n1 = r#"
    ---
	macro: `concat!()`
	- Joins constant string expressions at compile time returning them as a string slice
	- Concatenates literals into a static string slice
	---"#;
    println!("{n1}");

    println!("Macro: concat!");
    let s17: &str = concat!("cargo ", "run");
    println!(" > s17: {s17}");
}

pub fn strings_extracting() {
    let n1 = r#"
    ---
	pod: Extracting from String
	- UTF-8 characters could be between 1 and 4 bytes
	---"#;
    println!("{n1}");

    println!("String Extraction");

    let s20: &str = "🦀🦀🦀🦀🦀";
    println!(" > &s20: {s20}");

    let s21: &str = &s20[..4];
    println!(" > &s20[..4]: {s21}");
}

pub fn strings_different_bytes() {
    let n1 = r#"
    ---
	pod: Different Byte Length
	- 🦀 is 8-byte long
	- Letters `the` are 1-byte long
	---"#;
    println!("{n1}");

    println!("String Bytes");

    let s22: &str = "The(1-byte) 🦀(8-bytes)";
    println!(" > s22: {s22}");
}

pub fn strings_iterating() {
    let n1 = r#"
    ---
	pod: Linear Time Finding
	- Finding a character takes linear time because we have to iterate over each character
	---"#;
    println!("{n1}");

    println!("Iterating Strings");

    let s1 = "Hello 🦀";
    println!(" > s1: {s1}");

    println!(" > .bytes()");
    for b1 in s1.bytes() {
        println!(" > {b1}");
    }
}

pub fn strings_graphemes() {
    let n1 = r#"
    ---
	pod: Grapheme Or Grapheme Cluster
	- A user-perceived character, which may consist of multiple Unicode code points
	- The smallest unit of a written language that users recognize as a single character
	---"#;
    println!("{n1}");

    println!("Grapheme");

    let s1 = "Hello 🦀";
    println!(" > s1: {s1}");

    println!(" > .chars()");
    for c1 in s1.chars() {
        println!(" > {c1}");
    }

    println!(" > .graphemes()");
    for c1 in s1.graphemes(true) {
        println!(" > {c1}");
    }
}

pub fn strings_and_functions() {
    println!("String Deref Coercion");

    let s1 = "Hello Ferris";
    println!(" > main: &str s1: {s1}");

    let f1: String = format_slice(s1);
    println!(" > main: f1: {f1}");

    let s2 = String::from("Hello 🦀");
    println!(" > main: String s2: {s2}");

    let f2: String = format_slice(&s2);
    println!(" > main: f2: {f2}");
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
