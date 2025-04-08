use rand::prelude::*;

pub fn dependencies() {
    println!("----------");
    println!("Dependencies");

    println!(" > crates.io");
    println!(" > [dependencies]");

    let timeout: i32 = rand::rng().random_range(100..500);
    println!(" > random timeout: {timeout}");

}