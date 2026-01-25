//! # Compound Data Types - Strings

use std::fmt::format;

fn strings_definition() {
    let n1 = r#"
    pod: Compound Data Types
    - They can store more than one simple value
    - Ordered sequence of characters
    ---
    pod: String
    - Variable length of strings
    ---
    pod: String Slice (&str)
    - Has a fixed size and cannot be mutated
    - Is basically a reference (pointers)
    ---"#;
    println!("{n1}");

    let s1: &str = "Hello, ";
    let s2: String = String::from("Ferris!");
    println!("Strings: {s1}{s2}");
}

fn strings_push_str() {
    println!("Strings: push_str");

    let mut s3 = String::from("Hello, ");
    println!(" > before s3: {s3}");

    s3.push_str("Ferris!");
    println!(" > after s3: {s3}");
}

fn strings_pop() {
    println!("Strings: pop");

    let mut s4 = String::from("Hello!");
    println!(" > before s4: {s4}");

    let c1: Option<char> = s4.pop();
    println!(" > after s4: {s4}");
    println!(" > popped c1: {:?}", c1);
}

fn strings_push() {
    println!("Strings: push");

    let mut s5 = String::from("Hello");
    println!(" > before s5: {s5}");

    s5.push('!');
    println!(" > after s5: {s5}");
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
}

fn strings_to_string() {
    println!("Strings: to_string");

    let n7: i32 = 1100;
    let s7: String = n7.to_string();
    println!(" > s7: {s7}");

    let c8: char = 'x';
    let s8: String = c8.to_string();
    println!(" > s8: {s8}");

    let s9: String = "Ferris The Crab".to_string();
    println!(" > s9: {s9}");
}

fn strings_new() {
    println!("Strings: new");

    let s10: String = String::new();
    println!(" > s10: '{s10}'");
}

fn strings_format() {
    let n1 = r#"
    pod: Macro: format!()
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
