//! # Option Enum

fn option_enum() {
    let n1 = r#"
    pod: Option Enum
    - Variants: None, Some(T)
    - Functions: unwrap()
    ---"#;
    println!("{n1}");

    let op1: Option<String> = None;
    let op2: Option<String> = Some(String::from("Ferris"));
    println!("Option Enum");
    println!(" > op1: {op1:?}");
    println!(" > op2: {op2:?}");
}

fn option_functions() {
    let op3: Option<&str> = Some("Hello, Ferris!");
    println!("Option Enum");
    println!(" > unwrap: {}", op3.unwrap());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_option_enum() {
        option_enum()
    }

    #[test]
    fn run_option_functions() {
        option_functions()
    }
}
