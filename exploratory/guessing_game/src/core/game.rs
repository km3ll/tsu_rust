use rand::RngExt;
use std::cmp::Ordering::{Equal, Greater, Less};
use std::io;

pub fn guessing_game() {
    println!("\nGuess the number.");
    let secret: u32 = rand::rng().random_range(1..=100);

    loop {
        println!("\nInput your guess:");
        let mut guess: String = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line.");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input '{}'", guess.trim());
                continue;
            }
        };

        match guess.cmp(&secret) {
            Greater => println!("Too high!"),
            Less => println!("Too low!"),
            Equal => {
                println!("You win!");
                break;
            }
        }
    }

    println!("\nThanks for playing.");
}
