fn functions() {
    let n1 = r#"
    pod: Functions
    - Program fragment or segment designed to perform a specific task
    ---
    pod: Dot-Index Notation
    - Used to access the members of tuples and arrays
    ---
    pod: Code Blocks
    - Can return values like functions
    - Useful to initialize variables and isolating small computations
    ---"#;
    println!("{n1}");
}

fn inputs() {
    let mut cmd = String::new();
    println!("Enter a command: ");

    //std::io::stdin()
    //    .read_line(&mut cmd)
    //    .expect("Failed to read command");

    cmd.trim();
    println!("You entered '{cmd}' command");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_functions() {
        functions()
    }

    #[test]
    fn run_inputs() {
        inputs()
    }
}
