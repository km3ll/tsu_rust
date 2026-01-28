//! # Match Statement

use rand::Rng;
use rand::prelude::ThreadRng;

fn match_statement() {
    let n1 = r#"
    pod: Match Statement
    - Control flow operator
    - Transfers control to a particular block of code (arms) based on the value of a variable
    - It is exhaustive and covers all possible cases
    - One of the arms should execute
    ---"#;
    println!("{n1}");

    let mut rng: ThreadRng = rand::rng();
    let number = rng.random_range(1..=200);

    println!("Match: number:");
    match number {
        1 => println!(" > is one"),
        2 | 3 => println!(" > is either two or three"),
        4..=100 => println!(" > is between four and one hundred"),
        _ => println!(" > is greater than one hundred"),
    }
}

fn match_grades() {
    let mut rng: ThreadRng = rand::rng();
    let marks = rng.random_range(0..=100);

    let mut grade: char = 'N';
    match marks {
        90..=100 => grade = 'A', // Excellent
        80..=89 => grade = 'B',  // Good
        70..=79 => grade = 'C',  // Satisfactory/Fair
        60..=69 => grade = 'D',  // Marginal/Passing
        _ => grade = 'F',        // Failure
    }

    println!("Match: marks: {marks}, grade: {grade}");
}

fn match_let() {
    let mut rng: ThreadRng = rand::rng();
    let marks = rng.random_range(0..=100);

    println!("Match:");
    let mut grade = match marks {
        90..=100 => {
            println!(" > excellent!");
            'A'
        }
        80..=89 => {
            println!(" > good!");
            'B'
        }
        70..=79 => {
            println!(" > satisfactory / fair");
            'C'
        }
        60..=69 => {
            println!(" > marginal passing");
            'D'
        }
        _ => {
            println!(" > failure");
            'F'
        }
    };

    println!(" > marks: {marks}, grade: {grade}");
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
