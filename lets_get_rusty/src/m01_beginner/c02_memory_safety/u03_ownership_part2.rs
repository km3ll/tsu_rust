//! # Ownership Part 2

fn print_string(p1: String) {
    println!(" > func: ownership moved to p1: {p1}");
    println!(" > func: p1 is dropped here")
} // p1 is dropped here

fn generate_string() -> String {
    let p1 = String::from("Ferris");
    println!(" > func: owner p1: {p1}");
    println!(" > func: ownership returned: p1");
    p1
}

fn print_integer(i1: i32) {
    println!(" > func: owner primitive i1: {i1}");
    println!(" > func: i1 is dropped here")
}

fn add_to_string(mut p1: String) -> String {
    println!(" > func: ownership moved to mut p1: {p1}");
    println!(" > func: appending to p1");
    p1.push_str(" is awesome!");
    println!(" > func: ownership returned p1");
    p1
}

pub fn ownership_pass_into_function() {
    let n1 = r#"
    ---
	pod: Ownership Moved Into Functions
	- Values are moved by default
	- Passing a variable into a function has the same effect as assigning one variable to another variable
	---"#;
    println!("{n1}");

    println!("Ownership Moved Into Functions");
    let s1: String = String::from("John");
    println!(" > main: owner s1: {s1}");

    print_string(s1);
    println!(" > main: moved s1");
    // println!("s1: {s1}"); error: borrow of moved value 's1'
}

pub fn ownership_clone_into_function() {
    let n1 = r#"
    ---
	pod: Ownership Cloned Before Calling Functions
	- We first clone s2 and then move ownership of that clone into the function
	---"#;
    println!("{n1}");

    println!("Ownership Cloned Before Function");
    let s2: String = String::from("Ferris");
    println!(" > main: owner s2: {s2}");

    println!(" > main: sending clone");
    print_string(s2.clone());
    println!(" > main: owner s2: {s2}");
}

pub fn ownership_out_of_function() {
    let n1 = r#"
    ---
	pod: Ownership Moved Out Of Functions
	- Ownership of the generated function is transferred to s3 (caller)
	---"#;
    println!("{n1}");

    println!("Ownership Moved Out Of Functions");
    let s3: String = generate_string();
    println!(" > main: ownership moved to s3");
    println!(" > main: new owner s3: {s3}");
}

pub fn ownership_in_and_out_functions() {
    let n1 = r#"
    ---
	pod: Ownership Into and Out Of Functions
	- Functions taking ownership and giving it back (push_str)
	---"#;
    println!("{n1}");

    println!("Ownership Into/Out Of Functions");
    let s1: String = String::from("Ferris");
    println!(" > main: owner s1: {s1}");

    let s2 = add_to_string(s1);
    println!(" > main: new owner s2: {s2}");
}

pub fn ownership_stack_datatypes() {
    let n1 = r#"
    ---
	pod: Stack-Only Data Types
	- The value of x: i32 was cloned into z
	---"#;
    println!("{n1}");

    println!("Stack-Only Data Types");
    let x: i32 = 30;
    println!(" > main: owner primitive x: {x}");

    print_integer(x);
    println!(" > main: samer owner x: {x}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_ownership_pass_into_function() {
        ownership_pass_into_function();
    }

    #[test]
    fn run_ownership_clone_into_function() {
        ownership_clone_into_function();
    }

    #[test]
    fn run_ownership_out_of_function() {
        ownership_out_of_function();
    }

    #[test]
    fn run_ownership_in_and_out_functions() {
        ownership_in_and_out_functions();
    }

    #[test]
    fn run_ownership_stack_datatypes() {
        ownership_stack_datatypes();
    }
}
