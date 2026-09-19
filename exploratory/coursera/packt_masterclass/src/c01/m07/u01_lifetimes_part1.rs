//! # Lifetimes - Part 1

/*
fn reference(value: i32) -> &i32 {
    &value
}
*/

/*
fn greater(x: &i32, y: &i32) -> &i32 {
    if x > y {
        x
    } else {
        y
    }
}
*/

fn lifetimes() {
    let n1 = r#"
    pod: Lifetime
    - Defines the scope for which a reference is valid
    - A reference variable must live long enough for the duration in which it's being referenced
    ---
    pod: Borrow Checker
    - Rust module that verifies lifetime-related issues
    ---
    pod: Dangling Reference
    - Trying to access a resource that has been deallocated
    - For instance: a function taking ownership of a value an returning a reference to it
    ---
    pod: Undetermined Lifetimes
    - The lifetime of a variable is not known at compile time
    - For instance: a function receives two references to return the greater reference
    ---"#;
    println!("{n1}");

    let v1: i32 = 10;

    // Dangling Reference
    // let ref_v1: &i32 = reference(v1);
    // println!("Lifetimes: reference v1: {ref_v1}");

    // Undetermined Lifetime
    // let x = 10;
    // let y = 20;
    // let ref_v2 = greater(&x, &y);
    // println!("Lifetimes: reference v2: {ref_v2}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_lifetimes() {
        lifetimes()
    }
}
