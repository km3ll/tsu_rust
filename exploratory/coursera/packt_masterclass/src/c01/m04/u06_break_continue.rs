//! # Break and Continue

use rand::Rng;
use rand::prelude::ThreadRng;

fn loops_break() {
    let n1 = r#"
    pod: Loops
    - `break` stops a loop
    - `continue` skips the current iteration and continues with the next
    ---"#;
    println!("{n1}");

    println!("Loops: break");
    let mut rng: ThreadRng = rand::rng();
    loop {
        let n = rng.random_range(1..=200);
        println!(" > n: {n}");
        if n % 2 == 0 {
            println!(" > break");
            break;
        }
    }
}

fn loops_continue() {
    println!("Loops: continue");
    let mut rng: ThreadRng = rand::rng();
    let mut even: Vec<i32> = vec![];

    loop {
        let n = rng.random_range(0..=1000);
        if (n % 2 > 0) {
            println!(" > continue: {n}");
            continue;
        }
        even.push(n);
        if even.len() == 3 {
            println!(" > break");
            break;
        }
    }

    println!(" > even: {:?}", even);
}

fn loops_return_value() {
    println!("Loops: return value");
    let mut rng: ThreadRng = rand::rng();
    let mut count: i32 = 0;

    let third: i32 = loop {
        let n = rng.random_range(0..=5000);
        if (n % 2 > 0) {
            println!(" > continue: {n}");
            continue;
        }
        count += 1;
        if (count == 3) {
            println!(" > break: {n}");
            break n;
        }
    };

    println!(" > third even: {third}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_loops() {
        loops_break()
    }

    #[test]
    fn run_loops_continue() {
        loops_continue()
    }

    #[test]
    fn run_loops_return_value() {
        loops_return_value()
    }
}
