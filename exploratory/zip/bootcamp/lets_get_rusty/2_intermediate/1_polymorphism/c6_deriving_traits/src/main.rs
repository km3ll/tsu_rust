#![allow(unused)]

// Using the derive attribute to implement the `Debug` trait.
// Debug is a trait implemented in the Rust standard library and 
// it is brought into scope automatically
#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    
    let p1 = Point { x: 3, y: 1 };
    let p2 = Point { x: 3, y: 1 };
    let p3 = Point { x: 5, y: 5 };

    // Print p1 in debug mode {:?}
    // Error: `Point` does not implement the `Debug` trait
    println!("{:?}", p1);

    // Comparing points
    // Error: binary operation == cannot be applied to Point
    // The PartialEq trait has not been implemented yet for Point
    println!("p1 == p2 : {}", p1 == p2);
    println!("p1 == p3 : {}", p1 == p3);

}