#![allow(unused)]
use std::io;

use credit_card::find_credit_card;

mod credit_card;

fn main() {

    env_logger::init();

    /*
    let mut name = String::new();
    println!("Enter your name: ");
    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read line");

    find_credit_card(name.as_str());
     */

    find_credit_card("Any");
    find_credit_card("Tom");
    find_credit_card("Bob");
    find_credit_card("Ann");

}