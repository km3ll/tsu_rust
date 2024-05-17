#![allow(unused)]

fn main() {

    let mut s1: String = String::from("pod");

    // immutable reference
    let ref1: &String = &s1;
    print_reference(ref1);
    // non-lexical lifetimes
    // ref1 is used up to this point

    // mutable reference
    // the variable itself has to be mutable
    let ref2: &mut String = &mut s1;
    add_to_reference(ref2);

    println!("Value of s1 is: {s1}");

}

// Instead of taking ownership this function is borrowing and reading a String
fn print_reference(p1: &String) {
    println!("Hello, {p1}");
}

// This function is taken a mutable reference instead of ownership
fn add_to_reference(p1: &mut String) {
    p1.push_str(" is awesome!")
}

/* Dangling reference
pub fn generate_string() -> &String { // Missing lifetime specifier
    let s: String = String::from("Ferris");
    &s; // Dangling reference
}
*/