//! # If Let and Nested If

fn mod_operator() {
    let n1 = r#"
    ---
    pod: Mod Operator `%`
    - Computes the reminder after dividing two numbers (remainder == 0 -> even)
    ---"#;
    println!("{n1}");

    println!("Mod operator");
    let r1 = rand::random_range(0..=100);
    let is_even = r1 % 2 == 0;
    println!(" > random r1: {r1} is even: {is_even}");
}

fn if_let() {
    println!("Conditional if-let");
    let r1 = rand::random_range(1..=100);
    println!(" > random r1: {r1}");

    let bool1: bool = if r1 % 2 != 0 {
        println!(" > is odd");
        true
    } else {
        println!(" > is event");
        false
    };
    println!(" > bool1: {bool1}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_mod_operator() {
        mod_operator()
    }

    #[test]
    fn run_if_let() {
        if_let()
    }
}
