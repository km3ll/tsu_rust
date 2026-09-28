#![allow(unused)]

fn main() {
    // creation
    // 16-bit integer
    let a1: i16 = 5;
    let a2:f32 = 5.0;

    // mutability
    let mut b: i32 = 5;
    b = 10;
    println!("Value of b is {b}");

    // shadowing
    let c: i32 = 10;
    let c: i32 = 20;
    println!("Value of c is {c}");

    // scope
    let d: i32 = 30;
    // inner scope
    {
        let d = 40;
        println!("Inner value of d is {d}");
    }
    println!("Outer value of d is {d}")
}
