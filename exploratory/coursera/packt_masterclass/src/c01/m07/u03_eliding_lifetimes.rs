//! # Eliding Lifetimes

fn lifetime_elision() {
    let n1 = r#"
    pod: Lifetime Elision
    - The compiler automatically infers the lifetimes of references
    ---
    pod: Lifetime Elision Rules
    - (1) Each parameter that is a reference, gets its own lifetime parameter
    - (2) If there is exactly one input lifetime parameter, that lifetime is assigned to all output lifetime parameters
    - (3) If there are multiple input lifetime parameters, but one of them is `&self` or `mut &self`, the lifetime of self is assigned to all output lifetime parameters
    ---"#;
    println!("{n1}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_lifetime_elision() {
        lifetime_elision()
    }
}
