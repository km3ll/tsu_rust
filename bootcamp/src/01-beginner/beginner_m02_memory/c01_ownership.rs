pub fn ownership() {
    /*
    {
        let s1: String = String::from("Hello"); // Heap allocated 
        //Error: s1 is not valid outside the inner scope
    } // s1 would be dropped here
     */
    let s1: String = String::from("Hello"); // Heap allocated 
    println!("Value of s1 is: {}", s1);
} // si is dropped

pub fn move_ownership() {
    let s1: String = String::from("Hello"); // Heap allocated 
    let s2: String = s1;
    
    //println!("Value of s1 is: {}", s1); // borrow of moved value
    println!("Value of s2 is: {}", s2);
}

pub fn clone_ownership() {
    let s1: String = String::from("Hello");
    let s2: String = s1.clone();
    println!("Value of s1 is: {s1}. Value of s2 is: {s2}    ");
}

pub fn clone_primitives() {
    let x: i32 = 10;
    let y: i32 = x;
    println!("Value of y is: {y}");
}