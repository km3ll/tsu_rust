//! # While and Simple Loops

use rand::Rng;
use rand::prelude::ThreadRng;

fn loops() {
    let n1 = r#"
    pod: Loop
    - Block of code that continuosly repeats until a certain condition is reached
    ---"#;
    println!("{n1}");

    loop {
        println!("Loops: simple break");
        break;
    }
}

fn loops_while() {
    let mut rng: ThreadRng = rand::rng();
    let mut is_even = true;

    println!("Loops: while");
    while is_even {
        println!(" > is even");
        is_even = rng.random_range(0..=100) % 2 == 0
    }
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
