#![allow(unused)]

use rand::Rng;
use std::io;

pub fn e02_constants() {
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("Three hours in seconds: {THREE_HOURS_IN_SECONDS}")
}

pub fn e01_mutability() {
    let mut x: i32 = 5;
    println!("The value of x is {x}");
    x = 10;
    println!("The new value of x is {x}");
}
