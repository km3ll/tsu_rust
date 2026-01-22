//! # Ownership, Primitive, and Non-Primitive Types

fn ownership() {
    let n1 = r#"
    pod: Ownership
    - Each value in Rust has a variable that's called its owner.
    - There can only be one owner at a time.
    - When the owner goes out of scope, the value will be dropped.
    ---
    pod: Move and Copy
    - Rust copies the values of primitive types by default
    - Rust moves values of non-primitive types by default
    - A move leads to a change in ownership
    - A reference does not change ownership but leads to borrowing
    ---
    pod: Primitive and Non-Primitive Types
    - Primitive types are stored directly on the stack, cannot be empty and have a fixed size
    - Non-primitive types are stored on the heap, can grow and can be empty
    ---
    pod: Scope
    - A block of code defined with curly braces {}
    - Code blocks can be associated with functions, conditions, conditional statements and loops
    ---"#;
    println!("{n1}");
}

fn ownership_copy_move() {
    let f1: f64 = 32.6;
    let f2: f64 = f1;
    println!("Ownership: copy f1: {f1} into f2: {f2}");

    let s1 = String::from("hello");
    let s2 = s1;
    println!("Ownership: move s1 into s2: {s2}");

    let s3 = s2.clone();
    println!("Ownership: clone s2: {s2} into s3: {s3}");
}

fn ownership_borrowing() {
    let n1 = r#"
    pod: References
    - Allow to borrow the value of another variable without changing the ownership
    ---"#;
    println!("{n1}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_ownership() {
        ownership()
    }

    #[test]
    fn run_ownership_copy_move() {
        ownership_copy_move()
    }

    #[test]
    fn run_ownership_borrowing() {
        ownership_borrowing()
    }
}
