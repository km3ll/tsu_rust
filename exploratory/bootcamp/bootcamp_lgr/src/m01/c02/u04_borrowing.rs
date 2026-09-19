//! # Borrowing

fn print_string(r1: &String) {
    println!("Borrowing: r1: {r1}");
}

fn add_to_string(r2: &mut String) {
    println!("Borrowing adding to reference: r2: {r2}");
    r2.push_str(" is awesome!");
}

pub fn borrowing_definition() {
    let n1 = r#"
	pod: Borrowing
	- The act of creating a reference
	- Why borrow? For performance or when ownership is not needed/desired
	---
	pod: References
	- Are pointers with rules/restrictions
	- Do not take ownership
	- Are created with ampersand `&`
	- Can be mutable/immutable
	- To declare a mutable reference the variable itself has to be mutable
	---
	pod: Borrowing Rules
	- At any given time, you can have either one mutable reference or any number of immutable references
	- References must always be valid
	---"#;
    println!("{n1}");
}

pub fn borrowing_immutable_reference() {
    let n1 = r#"
	pod: Borrowing Mutable/Immutable References
	- When mutable, the borrowed type must also be mutable
	---"#;
    println!("{n1}");

    let s1: String = String::from("Ferris");
    let r1: &String = &s1;
    println!("Immutable reference: r1: {r1}");
    print_string(r1);
}

pub fn borrowing_mutable_reference() {
    let n1 = r#"
	pod: Automatic Dereferencing
	- We don't need to dereference r2
	- The asterisk `*` is a dereference operator `(*r2).push_str(" is awesome!")`
	---"#;
    println!("{n1}");

    let mut s2: String = String::from("Ferris");
    let r2: &mut String = &mut s2;
    println!("Mutable reference: r2: {r2}");
    add_to_string(r2);
}

pub fn borrowing_dangling_reference() {
    /**
     * fn generate_string() -> &String {
     *   let s: String = String::from("Ferris");
     *   &s //dangling reference
     * } // s is dropped
     *
     * let s3 = generate_string();
     */
    let n1 = r#"
	pod: Dangling Reference
	- A reference that points to memory that has already been freed, moved or gone out of scope
	- The referenced data is no longer valid
	- Rust prevents this at compile time
	---"#;
    println!("{n1}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_borrowing_definition() {
        borrowing_definition();
    }

    #[test]
    fn run_borrowing_immutable_reference() {
        borrowing_immutable_reference();
    }

    #[test]
    fn run_borrowing_mutable_reference() {
        borrowing_mutable_reference();
    }

    #[test]
    fn run_borrowing_dangling_reference() {
        borrowing_dangling_reference();
    }
}
