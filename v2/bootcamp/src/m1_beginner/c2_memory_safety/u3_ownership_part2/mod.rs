pub fn ownership_part2() {
    println!("----------");
    println!("Ownership Part 2");

    /**
     * Moving ownership INTO functions
     * - values are moved by default
     * - passing a variable into a function has the same effect 
     *   as assigning one variable to another variable 
     */
    let s1: String = String::from("John");
    print_string(s1);
    // println!("s1: {s1}"); error: borrow of moved value 's1'

    /**
     * Cloning
     * - instead of moving ownership, we first cloine s2 and then
     *   move ownership of that clone into the function
     */
    let s2: String = String::from("Wick");
    print_string(s2.clone());
    println!("s2: {s2}"); 

    /**
     * Moving ownership OUT of functions
     * - ownership of the generated function is tranferred to s3
     */
    let s3: String = generate_string();
    println!("s3: {s3}");

    /**
     * Functions taking ownership and giving it back
     */
    let s4: String = String::from("Ferris");
    let s5: String = add_to_string(s4);
    println!("s5: {s5}");

    /**
     * Stack-Only data types
     * - When we call print_integer the value of z is cloned into p2
     */
    let z: i32 = 30;
    print_integer(z);
}

fn print_string(p1: String) {
    println!("{p1}");
} // name is dropped

fn generate_string() -> String {
    String::from("Ferris")
}

fn add_to_string(mut p1: String) -> String {
    //p1.push_str(" is awesome!"); error: cannot borrow 'pa' as mutable
    p1.push_str(" is awesome!"); 
    p1
}

fn print_integer(p2: i32) {
    println!("p2: {p2}");
}