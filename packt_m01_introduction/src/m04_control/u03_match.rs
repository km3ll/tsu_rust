//! # Match Statement

use rand::Rng;
use rand::prelude::ThreadRng;

#[derive(Debug)]
enum Grade {
    A,
    B,
    C,
    D,
    F,
}

fn match_statement() {
    let n1 = r#"
    ---
    pod: Match Statement
    - Control flow operator
    - Transfers control to a particular block of code (`arm`) based on the value of a variable
    - It is `exhaustive` and covers all possible cases
    - One of the arms should execute
    ---"#;
    println!("{n1}");

    println!("Match number");
    let mut rng: ThreadRng = rand::rng();
    let number = rng.random_range(1..=200);
    println!(" > number");
    match number {
        1 => println!(" > is one"),
        2 | 3 => println!(" > is either two or three"),
        4..=100 => println!(" > is between four and one hundred"),
        _ => println!(" > is greater than one hundred"),
    }
    println!(" > {number}");
}

fn match_grades() {
    println!("Match marks");
    let mut rng: ThreadRng = rand::rng();

    let marks = rng.random_range(0..=100);
    println!(" > marks: {marks}");

    let mut grade: char = 'N';
    match marks {
        90..=100 => grade = 'A', // Excellent
        80..=89 => grade = 'B',  // Good
        70..=79 => grade = 'C',  // Satisfactory/Fair
        60..=69 => grade = 'D',  // Marginal/Passing
        _ => grade = 'F',        // Failure
    }

    println!(" > grade: {grade}");
}

fn match_let() {
    println!("Match mark blocks");
    let mut rng: ThreadRng = rand::rng();

    let marks = rng.random_range(0..=100);
    println!(" > marks: {marks}");

    let mut grade: Grade = match marks {
        90..=100 => {
            println!(" > excellent!");
            Grade::A
        }
        80..=89 => {
            println!(" > good!");
            Grade::B
        }
        70..=79 => {
            println!(" > satisfactory / fair!");
            Grade::C
        }
        60..=69 => {
            println!(" > marginal passing!");
            Grade::D
        }
        _ => {
            println!(" > failure!");
            Grade::F
        }
    };

    println!(" > grade: {grade:?}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_match_statement() {
        match_statement()
    }

    #[test]
    fn run_match_grades() {
        match_grades()
    }

    #[test]
    fn run_match_let() {
        match_let()
    }
}
