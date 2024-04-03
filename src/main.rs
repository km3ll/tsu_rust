// pod: disables the warning over unused function
#![allow(unused)]

// pod: std = standard
use rand::Rng;
use std::cmp::Ordering;
use std::io;

// pod: the main function is always the first code that runs.
fn main() {
    greet();
    start_game();
}

fn log(msg: &str) {
    // pod: using a ! means that you’re calling a macro instead of a normal function
    println!("pod042: {}", msg);
}

fn greet() {
    log("Tactical Support Unit Pod 042.");
}

fn start_game() {
    // pod: a particular random number generators we're going to use
    let secret: i32 = rand::thread_rng()
        // pod: range expression is inclusive on the lower and upper bounds
        .gen_range(1..=100);
    loop {

        println!("Please input your guess:");

        // pod: mutable var
        let mut guess: String = String::new();
        io::stdin()
            // pod: the & indicates that this argument is a reference,
            .read_line(&mut guess)
            // pod: result is an enumeration. each possible state a variant.
            //      result’s variants are Ok and Err.
            .expect("Failed to read line");
        println!("You guessed: {guess}");

        // pod: The trim method eliminates newline characters
        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            // pod: tell the program to go to the next iteration of the loop
            Err(_) => continue
        };

        match guess.cmp(&secret) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You win");
                break;
            }
        }

    }
}