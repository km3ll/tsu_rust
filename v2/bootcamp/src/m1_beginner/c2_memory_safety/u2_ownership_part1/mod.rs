pub fn ownership_part1() {
    println!("----------");
    println!("Ownership Part 1");

    /**
     * Ownership
     * Strategy to manage memory through a set of rules checked at compile time
     * 
     * Rules
     * - Each value has a variable that is called its 'owner'
     * - There can only be one owner at a time
     * - When the owner gets out of scope, the value is dropped
     */
    println!("Ownership");

    /**
     * s1: stack frame (pointer) to the value in the heap
     * Pod: actual string allocated on the heap
     * 
     * s1 is the owner of the data stored on the heap
     * when s1 goes out of scope, then the data is cleaned up
     */
    println!("Scope");
    let s1: String = String::from("Pod"); // heap-allocated String
    println!("s1: {s1}");

    {
        let s2: String = String::from("Pod"); 
    } // s2 is dropped
    //println!("s2: {s2}"); error: cannot find value 's2' in this scope

    /**
     * Moving ownership
     * - values are moved by default
     * - s3 is invalidated
     * - s4 is now the owner of the String
     */
    println!("Moving ownership");
    let s3: String = String::from("Pod"); 
    let s4: String = s3;
    // println!("s3: {s3}"); error: borrow of moved value 's3'
    println!("s4: {s4}");

    /**
     * Cloning values
     * - s5 has his own copy of s4 String
     */
    let s5: String = s4.clone();
    println!("s5: {s5}");

    /**
     * Primitive types
     * - Primitives that are entirely stored on the stack such as:
     *   integers, floating point numbers, booleans or characters 
     *   are cloned by default.
     * - These types are cheap to clone so there's no material 
     *   difference between cloning and moving the values.
     */
    let x: i8 = 10;
    let y: i8 = x;
    println!("x: {x}");
} // s1 is dropped 