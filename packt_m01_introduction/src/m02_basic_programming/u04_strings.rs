//! # Compound Data Types - Strings

use std::fmt::format;

fn strings_definition() {
    let n1 = r#"
    ---
    pod: Compound Data Types
    - They can store more than one simple value
    - Ordered sequence of characters

    pod: String
    - Variable length of strings

    pod: String Slice `&str`
    - Has a fixed size and cannot be mutated
    - Is basically a reference (pointer)
    ---"#;
    println!("{n1}");

    println!("Strings");
    let s1: &str = "Hello, ";
    let s2: String = String::from("Ferris!");
    println!(" > &str: {s1}, String: {s2}");
}

fn strings_push_str() {
    let n1 = r#"
    ---
    method: `push_str()`
    - Appends a given string slice onto the end of this String.

    method: `pop()`
    - Removes the last character from the string buffer and returns it.
    - Returns None if this String is empty.

    method: `push()`
    - Appends the given char to the end of this String.

    method: `is_empty()`
    - Returns true if this String has a length of zero, and false otherwise.

    method: `len()`
    - Returns the length of this String, in bytes, not chars or graphemes.
    - In other words, it might not be what a human considers the length of the string.

    method: `contains()`
    - Returns true if the given pattern matches a sub-slice of this string slice.
    - Returns false if it does not.

    method: `capacity()`
    - Returns this String's capacity, in bytes.

    method: `with_capacity()`
    - Creates an empty String, but one with an initial buffer that can hold at least capacity bytes.
    - If the given capacity is 0, no allocation will occur, and this method is identical to the `new` method.

    method: `trim()`
    - Returns a string slice with leading and trailing whitespace removed.

    method: `new()`
    - Given that the String is empty, this will not allocate any initial buffer.
    - While that means that this initial operation is very inexpensive, it may cause excessive allocation later when you add data.
    ---"#;
    println!("{n1}");

    println!("Strings: push_str");

    let mut s3 = String::from("Hello, ");
    println!(" > before mut s3: {s3}");

    s3.push_str("Ferris!");
    println!(" > after mut s3: {s3}");
}

fn strings_pop() {
    println!("Strings: pop");

    let mut s4 = String::from("Hello!");
    println!(" > before mut s4: {s4}");

    let c1: Option<char> = s4.pop();
    println!(" > after mut s4: {s4}");
    println!(" > popped c1: {c1:?}");
}

fn strings_push() {
    println!("Strings: push");

    let mut s5 = String::from("Hello");
    println!(" > before mut s5: {s5}");

    let c1 = '!';
    s5.push(c1);
    println!(" > char c1: {c1}");
    println!(" > after mut s5: {s5}");
}

fn strings_functions() {
    println!("Strings: functions");

    let s6: String = String::from("Hello Ferris!   ");
    println!(" > s6: '{s6}'");

    println!(" > is_empty(): {}", s6.is_empty());
    println!(" > len(): {}", s6.len());
    println!(" > contains(): {}", s6.contains("xy"));
    println!(" > capacity(): {}", s6.capacity());
    println!(" > trim(): {}", s6.trim());

    let s6 = String::with_capacity(10);
    println!(" > with_capacity(10): {s6}");
}

fn strings_to_string() {
    println!("Strings: to_string");

    let n7: i32 = 1100;
    let s7: String = n7.to_string();
    println!(" > i32: {s7}");

    let c8: char = 'x';
    let s8: String = c8.to_string();
    println!(" > char: {s8}");

    let s9: String = "Ferris The Crab".to_string();
    println!(" > &str: {s9}");
}

fn strings_new() {
    println!("Strings: new");

    let s10: String = String::new();
    println!(" > s10: '{s10}'");
}

fn strings_format() {
    let n1 = r#"
    ---
    macro: `format!()`
    - Combines input strings by replacing placeholders with their values
    ---"#;
    println!("{n1}");

    println!("Strings: format");
    let s11 = format!("{} the {}", "Ferris", "Crab");
    println!(" > s11: {s11}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_strings_definition() {
        strings_definition()
    }

    #[test]
    fn run_strings_push_str() {
        strings_push_str()
    }

    #[test]
    fn run_strings_pop() {
        strings_pop()
    }

    #[test]
    fn run_strings_push() {
        strings_push()
    }

    #[test]
    fn run_strings_functions() {
        strings_functions()
    }

    #[test]
    fn run_strings_to_string() {
        strings_to_string()
    }

    #[test]
    fn run_strings_new() {
        strings_new()
    }

    #[test]
    fn run_strings_format() {
        strings_format()
    }
}
