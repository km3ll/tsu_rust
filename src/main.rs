// pod: disables the warning over unused function
#![allow(unused)]

// pod: std = standard
use std::io;

mod game;

// pod: the main function is always the first code that runs.
fn main() {
    greet("as");
    run_game();
}

fn log(msg: &str) {
    // pod: using a ! means that you’re calling a macro instead of a normal function
    println!("pod042: {}", msg);
}

fn greet(msg: &str) {
    log("Tactical Support Unit Pod 042.");
}

fn run_game() {

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

}