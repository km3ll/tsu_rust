use std::fmt::format;

fn string_definition() {
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

fn string_functions() {
    println!("Strings: push_str()");
    let mut s3 = String::from("Hello, ");
    println!(" > before: {s3}");
    s3.push_str("Ferris!");
    println!(" > after: {s3}");

    println!("Strings: pop()");
    let mut s4 = String::from("Hello!");
    println!(" > before: {s4}");
    s4.pop();
    println!(" > after: {s4}");

    println!("Strings: push()");
    let mut s5 = String::from("Hello");
    println!(" > before: {s5}");
    s5.push('!');
    println!(" > after: {s5}");

    let s6: String = String::from("Hello Ferris!   ");
    println!("Strings: '{s6}'");
    println!(" > is_empty(): {}", s6.is_empty());
    println!(" > len(): {}", s6.len());
    println!(" > contains(): {}", s6.contains("xy"));
    println!(" > capacity(): {}", s6.capacity());
    println!(" > trim(): {}", s6.trim());

    let n7: i32 = 1100;
    let s7: String = n7.to_string();

    let c8: char = 'x';
    let s8: String = c8.to_string();
    println!("Strings: to_string(): s7: {s7}, s8: {s8}");

    let s9: String = "Ferris The Crab".to_string();
    println!("Strings: to_string: s9: {s9}");

    let s10: String = String::new();
    println!("Strings: new(): s10: '{s10}', len(): {}", s10.len());

    let n1 = r#"
    pod: Macro: format!()
    - Combines input strings by replacing placeholders with their values
    ---"#;
    println!("{n1}");

    let s11 = format!("{} the {}", "Ferris", "Crab");
    println!("Strings: format!(): s11: {s11}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_string_definition() {
        string_definition()
    }

    #[test]
    fn run_string_functions() {
        string_functions()
    }
}
