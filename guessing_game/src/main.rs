// std = standard library
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {

    let secret: u32 = rand::thread_rng()
        .gen_range(1..=100);

    println!("[Guess the number]");
    println!("042: Please input your guess:");

    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("042: Failed to read line");

    let guess: u32 = guess
        .trim().parse()
        .expect("042: Please type a number");

    println!("042: You guessed: {guess}");

    match guess.cmp(&secret) {
        Ordering::Less => println!("042: Too small"),
        Ordering::Greater => println!("042: Too big"),
        Ordering::Equal => println!("[You win!]")
    }

}
