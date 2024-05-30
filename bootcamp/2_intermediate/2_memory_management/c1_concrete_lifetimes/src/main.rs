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