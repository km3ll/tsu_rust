//! # Concrete Lifetimes

fn concrete_lifetimes() {
    let n1 = r#"
    pod: Concrete Lifetimes
    - The time during which a value exists at a memory location
    - For instance: heap-allocated Strings
    ---"#;
    println!("{n1}");
}

fn concrete_lifetimes_scope() {
    let s1 = String::from("Hello, pod!");
    {
        let s2 = "Hello, Ferris!".to_owned();
    } // lifetime of s2 ends here
    println!("s1: {}", s1);
}

fn concrete_lifetimes_moving() {
    let s1 = String::from("Hello, pod!");
    let s2 = s1; // lifetime of s1 ends here

    // println!("s1: {}", s1); error: borrow of moved value
    println!("s2: {}", s2);
}

fn concrete_lifetimes_references() {
    let r1: &String;
    {
        let s1 = String::from("Hello, Ferris!");
        r1 = &s1;
        println!("r1: {}", r1);
        // r1 = &s1; error: s1 does not live long enough
    }
    // println!("r1: {}", r1);
}

fn concrete_lifetimes_non_lexical() {
    let n1 = r#"
    pod: Non-Lexical Lifetimes
    - Lifetimes that are not tied to scope
    - r1 and r2 do not live for the entire duration of the scope. So there is no conflict between mutable and immutable references 
    ---"#;
    println!("{n1}");

    let mut s1 = String::from("Hello, Ferris");
    let r1 = &s1;
    println!("r1: {}", r1); // lifetime of r1 ends here

    let r2 = &mut s1;
    r2.push_str("!"); // lifetime of r2 ends here

    // println!("r1: {}", r1); error: Cannot borrow r2 as mutable (lifetimes overlap)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_concrete_lifetimes() {
        concrete_lifetimes()
    }

    #[test]
    fn run_concrete_lifetimes_scope() {
        concrete_lifetimes_scope();
    }

    #[test]
    fn run_concrete_lifetimes_moving() {
        concrete_lifetimes_moving();
    }

    #[test]
    fn run_concrete_lifetimes_references() {
        concrete_lifetimes_references();
    }

    #[test]
    fn run_concrete_lifetimes_non_lexical() {
        concrete_lifetimes_non_lexical();
    }
}
