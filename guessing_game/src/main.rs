// std = standard library
use std::io;

fn main() {

    println!("042: Guess the number");
    println!("042: Please input your guess:");

    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("042: Failed to read line");

    println!("042: You guessed: {guess}")

}
