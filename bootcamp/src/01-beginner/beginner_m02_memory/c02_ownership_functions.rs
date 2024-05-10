pub fn main_ownership() {
    
    let s1: String = String::from("Pod");
    ownership_into(s1.clone());

    /*
    print_string(s1); // Ownership of s1 is moved to p1
    println!("Value of s1 is: {s1}"); // Error: borrow of moved value
    */

    let s2: String = ownership_from();
    println!("Value of s2 is: {s2}");

    let s3: String = String::from("Saitama");
    let s4 = ownership_take_and_give_back(s3);
    println!("Value of s4 is: {s4}");

    ownership_stack_only_data_type(20);

}

pub fn ownership_into(p1: String) {
    // p1 lives under the scope of this function's scope
    println!("Value of p1 is: {p1}");
} // p1 is dropped here

pub fn ownership_from() -> String {
    String::from("Ferris")
}

// To borrow ownership p1 is marked as mutable
pub fn ownership_take_and_give_back(mut p1: String) -> String {
    p1.push_str(" is awesome!");
    p1
}

pub fn ownership_stack_only_data_type(i: i32) {
    println!("Value of i is: {i}")
}