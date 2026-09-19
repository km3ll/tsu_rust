//! # If Let and Nested If

fn mod_operator() {
    let n1 = r#"
    pod: Mod Operator
    - The % operator computes the reminder after dividing two numbers (remainder == 0 -> even)
    ---"#;
    println!("{n1}");

    let is_even = 10 % 2 == 0;
    println!("Mod Operator: is_even: {is_even}");
}

fn if_let() {
    let is_odd: bool = if 9 % 2 == 0 {
        println!("If-let: number is odd");
        true
    } else {
        println!("If-let: number is even");
        false
    };
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
