//! # Break and Continue

use rand::Rng;
use rand::prelude::ThreadRng;

fn loops_break() {
    let n1 = r#"
    ---
    pod: Loops
    - `break` stops a loop
    - `continue` skips the current iteration and continues with the next
    ---"#;
    println!("{n1}");

    println!("Loop");
    let mut rng: ThreadRng = rand::rng();
    loop {
        let n = rng.random_range(1..=200);
        println!(" > n: {n}");
        if n % 2 == 0 {
            println!(" > break (is_even)");
            break;
        }
    }
}

fn loops_continue() {
    println!("Loop");
    let mut rng: ThreadRng = rand::rng();
    let mut even: Vec<i32> = vec![];

    loop {
        let n = rng.random_range(0..=1000);
        println!(" > n: {n}");
        if (n % 2 > 0) {
            println!(" > continue (is_odd)");
            continue;
        }
        even.push(n);
        if even.len() == 3 {
            println!(" > break");
            break;
        }
    }
}

fn loops_return_value() {
    println!("Loop");
    let mut rng: ThreadRng = rand::rng();
    let mut count: i32 = 0;

    let third: i32 = loop {
        let n = rng.random_range(0..=5000);
        println!(" > n: {n}");
        if (n % 2 > 0) {
            println!(" > continue (is_odd)");
            continue;
        }
        count += 1;
        println!(" > even #{count}");
        if (count == 3) {
            println!(" > break");
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
