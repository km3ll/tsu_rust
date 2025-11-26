use rand::Rng;
use std::io;
use std::cmp::Ordering::{Equal, Greater, Less};

pub fn guessing_game() {
    println!("[ Guess the number ]");
    let secret_number: u32 = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess");
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) =>  num,
            Err(_) => continue
        };

        match guess.cmp(&secret_number) {
            Less => println!("Too small"),
            Greater => println!("Too big"),
            Equal => {
                println!("You win!");
                break;
            },
        }
    }
}