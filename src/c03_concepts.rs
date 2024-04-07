#![allow(unused)]

use rand::Rng;
use std::io;

pub fn shadowing_02() {
    // Because we’re effectively creating a new variable when we use the let keyword again,
    // we can change the type of the value but reuse the same name
    let spaces: &str = "    ";
    println!("Value of spaces: '{spaces}'");
    let spaces: usize = spaces.len();
    println!("Value of spaces: '{spaces}'");
}
pub fn shadowing_01() {
    let x: i32 = 5;
    println!("Value of x: {x}");
    let x: i32 = x + 1;
    println!("Shadowed value of x: {x}");
    {
        let x: i32 = x * 2;
        println!("Shadowed value of x in inner scope: {x}")
        // When that scope is over, the inner shadowing ends and x returns to being 6
    }
    println!("Final value of x: {x}")
}

pub fn constants() {
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("Three hours in seconds: {THREE_HOURS_IN_SECONDS}")
}

pub fn mutability() {
    let mut x: i32 = 5;
    println!("The value of x is {x}");
    x = 10;
    println!("The new value of x is {x}");
}