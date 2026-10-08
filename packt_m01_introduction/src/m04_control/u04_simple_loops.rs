//! # While and Simple Loops

use rand::Rng;
use rand::prelude::ThreadRng;

fn loops() {
    let n1 = r#"
    ---
    pod: Loop
    - Block of code that continuosly repeats until a certain condition is reached
    ---"#;
    println!("{n1}");

    println!("Loop");
    loop {
        println!(" > simple `break`");
        break;
    }
}

fn loops_while() {
    println!("While");
    let mut rng: ThreadRng = rand::rng();
    let mut is_even = true;

    println!(" > is even");
    while is_even {
        println!(" > {is_even}");
        is_even = rng.random_range(0..=100) % 2 == 0
    }
    println!(" > {is_even}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_loops() {
        loops()
    }

    #[test]
    fn run_loops_while() {
        loops_while()
    }
}
