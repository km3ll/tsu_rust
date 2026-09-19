#![allow(unused)]

fn main() {
    // scalar data types

    // boolean
    let truthy: bool = true;
    let falsy: bool = false;

    // 5 types of unsigned integers (positive)
    let u1: u8 = 1;
    let u2: u16 = 2;
    let u3: u32 = 3;
    let u4: u64 = 4;
    let u5: u128 = 5;

    // 5 types of signed integers (negative or positive)
    let i6: i8 = -6;
    let i7: i16 = 7;
    let i8: i32 = -8;
    let i9: i64 = 9;
    let i10: i128 = -10;

    // 2 types of floating point numbers (decimal places)
    let f11: f32 = 11.0;
    let f12: f64 = 12.0;

    // 2 types of platform specific integers
    let p13: usize = 13;
    let p14: usize = 14;

    // 3 types of characters
    let c15: char = 'k';
    let s16: &str = "hello"; // String slices
    let s17: String = String::from("hello");

    // compound data types

    // arrays
    let a17: [i32; 5] = [1, 2, 3, 4, 5];
    let last: i32 = a17[4];

    // tuples
    let t18: (i32, i32, i32) = (1, 2, 3);
    let t19: (i32, f64, &str) = (5, 5.0, "5");
    let value: f64 = t19.1; // 5.0
    let (value1, value2, value3): (i32, f64, &str) = t19;

    // empty tuple = unit type
    // usually returned implicitly when no other meaningful value could be returned
    let unit: () = ();

    // type alias
    type Age = u8;
    let a1: Age = 39;

    run_example();
}

fn run_example() {
    let my_first_initial: char = 'C';
    if my_first_initial.is_alphabetic() {
        println!("Alphabetical!");
    } else if my_first_initial.is_numeric() {
        println!("Numerical!");
    } else {
        println!("Neither alphabetic nor numeric!");
    }
}