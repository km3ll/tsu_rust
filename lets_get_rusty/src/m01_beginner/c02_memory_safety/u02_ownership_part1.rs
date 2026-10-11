//! # Ownership Part 1

pub fn ownership_strategy() {
    let n1 = r#"
    ---
	pod: Ownership
	- A strategy to manage memory through a set of rules checked at compile time

	pod: Ownership Rules
	- 1. Each value has a variable that is called its `owner`
	- 2. There can only be one owner at a time
	- 3. When the owner gets out of scope, the value is dropped
	---"#;
    println!("{n1}");
}

pub fn ownership_scope() {
    let n1 = r#"
    ---
	pod: Ownership Scope
	- The actual string allocated on the heap
	- s1 is a stack frame (pointer) to the value in the heap
	- s1 is the owner of the data stored on the heap
	- When s1 goes out of scope, then the data is cleaned up
	---"#;
    println!("{n1}");

    println!("Ownership");
    let s1: String = String::from("Ferris"); // heap-allocated String
    println!(" > outer: owner s1: {s1}");

    {
        let s2: String = String::from("Rust");
        println!(" > inner: owner s2: {s2}");
    } // s2 is dropped
    println!(" > outer: s2 was dropped");

    // println!("s2: {s2}"); error: cannot find value 's2' in this scope
    println!(" > outer: value on the heap s1: {s1}");
}

pub fn ownership_moving() {
    let n1 = r#"
    ---
	pod: Moving Ownership
	- Values are moved by default
	- s3 is invalidated
	- s4 is now the owner of the String
	---"#;
    println!("{n1}");

    println!("Moved Ownership");
    let s3: String = String::from("Ferris");
    println!(" > owner s3: {s3}");

    let s4: String = s3;
    println!(" > new owner s4: {s4}");
    // println!("s3: {s3}"); error: borrow of moved value 's3'
    println!(" > error: borrow of moved value s3");
}

pub fn ownership_cloning() {
    let n1 = r#"
    ---
	pod: Ownership Clone
	- By executing `s4.clone()` s5 has his own copy of s4 String
	---"#;
    println!("{n1}");

    println!("Ownership Clone");
    let s4: String = String::from("Rust");
    println!(" > owner s4: {s4}");

    let s5: String = s4.clone();
    println!(" > cloned value's new owner s5: {s5}");
}

pub fn ownership_primitive_types() {
    let n1 = r#"
    ---
	pod: Ownership of Primitive Types
	- Primitives that are entirely stored on the stack
	- Integers, floating point numbers, booleans or characters are cloned by default
	- There's no material difference between cloning and moving the values
	---"#;
    println!("{n1}");

    println!("Ownership of Primitive Types");
    let x: i8 = 10;
    println!(" > primitive x: {x}");

    let y: i8 = x;
    println!(" > cloned primitive y: {y}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_ownership_strategy() {
        ownership_strategy();
    }

    #[test]
    fn run_ownership_scope() {
        ownership_scope();
    }

    #[test]
    fn run_ownership_moving() {
        ownership_moving();
    }

    #[test]
    fn run_ownership_cloning() {
        ownership_cloning();
    }

    #[test]
    fn run_ownership_primitive_types() {
        ownership_primitive_types();
    }
}
