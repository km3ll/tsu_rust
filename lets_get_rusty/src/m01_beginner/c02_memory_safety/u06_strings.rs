//! # Strings in Rust

use unicode_segmentation::UnicodeSegmentation;

fn format_it(p1: &str) -> String {
    format!("[{}]", p1)
}

pub fn strings_acronyms() {
    let n1 = r#"
	pod: ASCII
	- American Standard Code for Information Interchange
	- An encoding which maps integers to characters
	---
	pod: UTF-8
	- Unicode Transformation Format (8 bits = 1 byte)
	---"#;
    println!("{n1}");
}

pub fn strings_utf_8() {
    let s1: &str = "Hello Ferris 🦀";
    println!("Slice: UTF-8 Encoded &str s1: {s1}");

    let s2: String = String::from("Hello 🦀");
    println!("String::from s2: {s2}");

    let s3: String = "Ferris 🦀".to_string();
    println!("Slice: to_string s3: {s3}");

    let s4: String = "🦀 Hello".to_owned();
    println!("Slice: to_owned s4: {s4}");

    let s5: &str = &s4[..];
    println!("Slice: from slice s5: {s5}");

    let mut s6: String = String::from("Hello ");
    s6.push_str("Ferris ");
    s6.push_str("🦀");
    println!("String: mutable push_str s6: {s6}");

    s6.replace_range(.., "Hello Pod!");
    println!("String: mutable replace_range s6: {s6}");
}

pub fn strings_slice() {
    let n1 = r#"
	pod: String Slice
	- Is a borrowed view of a sequence of UTF-8 encoded bytes
	- Does not own the underlying data, just references part (or all) of a String
	---"#;
    println!("{n1}");
}

pub fn strings_string() {
    let n1 = r#"
	pod: String Type
	- Is a owned, growable, heap-allocated UTF-8 text type
	---"#;
    println!("{n1}");
}

pub fn strings_concatenation() {
    let s7: String = String::from("Hello ");
    let s8: String = String::from("Ferris");
    let s9 = s7 + &s8;
    println!("String: plus operator (+) s9: {s9}");

    let n1 = r#"
	pod: macro: format!()
	- Creates a String using interpolation and formatting
	- It just returns the formatted text as an owned String
	- Less efficient because it copies the contents of strings
	- Can take string and string slices as parameters
	---"#;
    println!("{n1}");

    let s11: String = String::from("tic");
    let s12: String = String::from("tac");
    let s13: &str = "toe";
    let s14: String = format!("{}, {}, {}", s11, s12, s13);
    println!("String format!() s14: {s14}");

    let n1 = r#"
	pod: macro: concat!()
	- Joins constant string expressions at compile time returning them as a string slice
	---"#;
    println!("{n1}");

    let s17: &str = concat!("cargo-", "run");
    println!("Slice concat!() s17: {s17}");
}

pub fn strings_extracting() {
    let n1 = r#"
	pod: Extracting from String
	- UTF-8 characters could be between 1 and 4 bytes
	---"#;
    println!("{n1}");

    let s20: &str = "🦀🦀🦀🦀🦀";
    let s21: &str = &s20[..4];
    println!("Extracting from String s21: {s21}");
}

pub fn strings_different_bytes() {
    let n1 = r#"
	pod: Different Byte Length
	- 🦀 is 8-byte long
	- Letters `the` are 1-byte long
	---"#;
    println!("{n1}");

    let s22: &str = "🦀the🦀";
    println!("Extracting bytes s22: {s22}");
}

pub fn strings_iterating() {
    let n1 = r#"
	pod: Linear Time
	- Finding a character takes linear time because we have to iterate over each character
	---"#;
    println!("{n1}");

    println!("Iterating over bytes .bytes()");
    for b1 in "Hello 🦀".bytes() {
        println!("> {b1}");
    }
}

pub fn strings_graphemes() {
    let n1 = r#"
	pod: Grapheme Or Grapheme Cluster
	- A user-perceived character, which may consist of multiple Unicode code points
	- The smallest unit of a written language that users recognize as a single character
	---"#;
    println!("{n1}");

    println!("Grapheme .chars()");
    for c1 in "Hello 🦀".chars() {
        println!("> {c1}");
    }

    println!("Grapheme .graphemes()");
    for c1 in "Hello 🦀".graphemes(true) {
        println!("> {c1}");
    }
}

pub fn strings_and_functions() {
    let s1 = "Hello Ferris";
    let s2 = String::from("Hello 🦀");
    let s3: String = format_it(s1);
    let s4: String = format_it(&s2);
    println!("Deref Coercion in parameters: s3: '{s3}', s4: '{s4}'");
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
