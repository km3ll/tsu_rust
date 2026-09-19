#![allow(unused)]
use std::io;

use credit_card::find_credit_card;

mod credit_card;

fn main() {

    /*
    let mut name = String::new();
    println!("Enter your name: ");
    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read line");

    find_credit_card(name.as_str());
    */
    find_credit_card("Amy");
    
}