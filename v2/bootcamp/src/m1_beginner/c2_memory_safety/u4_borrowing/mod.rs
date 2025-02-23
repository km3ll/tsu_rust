pub fn borrowing() {
    println!("----------");
    println!("Borrowing");

    /**
     * Borrowing
     * - The act of creating a reference
     * - References are pointers with rules/restrictions
     * - References do not take ownership
     * 
     * Why borrow?
     * - Performance
     * - When ownership is not needed/desired
     * 
     * Rules
     * - At any given time, you can have either one mutable reference
     *   or any number of immutable references
     * - References must always be valid
     * 
     * References
     * - Created with an ampersand
     */
    let s1: String = String::from("John");

    /**
     * Immutable reference
     */
    let r1: &String = &s1;
    print_string(r1);
    println!("s1: {s1}");

    /**
     * Mutable reference
     * - In order to declare a mutable reference the variable itself
     *   has to be mutable
     */
    let mut s2: String = String::from("Ferris");
    let r2: &mut String = &mut s2;
    add_to_string(r2);
    println!("s2: {s2}");

    // Dangling reference
    // let s3 = generate_string();
}

/**
 * Borrowing
 * - Function does not need to take ownership of the string
 */
fn print_string(r1: &String) {
    println!("r1: {r1}");
}

fn add_to_string(r2: &mut String) {
    r2.push_str(" is awesome!");
}

/*
fn generate_string() -> &String {
    let s: String = String::from("Ferris");
    &s //dangling reference
} // s is dropped
*/