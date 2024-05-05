#![allow(unused)]
use rand::Rng;

pub fn print_age(age: i32) {
    println!("You are {} years old.", age);
}

pub fn get_age() -> i32 {
    let age: i32 = rand::thread_rng().gen_range(18..=80);
    age
}