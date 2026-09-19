fn main() {

    // The borrow checker tracks the lifetime of values and checks to make sure
    // that references are valid at compile time.

    // Concrete Lifetimes
    // A concrete lifetime is the time during which a value exists at a particular
    // memory location. A lifetime starts when a value is created or moved into a
    // particular memory location and ends when a value is dropped or moved out 
    // or a particular memory location.

    // heap-allocated strings
    case_1();
    case_2();
    case_3();

    // references
    case_4();
    case_5();

}

pub fn case_5() {

    // The borrowing rules state that there can only be either one mutable
    // mutable reference or any number of immutable references at a given time.
    
    // Non-Lexical Lifetimes
    // References r1 and r2 do not live for the entire duration of this scope.

    // heap-allocated string
    let mut s1: String = String::from("Let's Get Rusty");
    
    // immutable reference
    let r1: &String = &s1;
    // let r2 = &mut s1; //Error: cannot borrow s2 as mutable because it's borrowed as immutable
    println!("r1: {r1}"); // last usage of r1

    // mutable reference
    let r2 = &mut s1;
    r2.push_str("!"); // last usage of r2
    println!("r2: {r2}");

}

pub fn case_4() {

    // String reference
    let r1: &String;

    {
        let s1: String = String::from("Let's Get Rusty!"); // s1 life starts here
        println!("s1: {s1}");   

        // The borrow checker checkt at compile time that the lifetime of a reference
        // is contained within lifetime of the value that it is borrowing.
        // r1 = &s1; Error: s1 does not live enough

        r1 = &s1; 
        println!("r1: {r1}");

    }// s1 life ends here

    // println!("r1: {r1}") Error: used binding is not initialized

}

pub fn case_3() {
    let s1: String = String::from("Let's Get Rusty!"); // s1 life starts here
    println!("s1: {s1}");
    let s2: String = s1; // s1 life ends here when the value is moved into s2
    println!("s2: {s2}");
    // println!("s1: {s1}"); Error: borrow of moved value s1
}

pub fn case_2() {
    // inner scope
    {
        let s1: String = String::from("Let's Get Rusty!"); // s1 life starts here
        println!("s1: {s1}");
    }
    // println!("s1: {s1}"); Error: cannot find value s1 in this scope
}

pub fn case_1() {
    // heap-allocated string
    let s1: String = String::from("Let's Get Rusty!"); // s1 life starts here
    println!("s1: {s1}");
}// s1 life ends here