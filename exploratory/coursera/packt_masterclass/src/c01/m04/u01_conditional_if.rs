//! # Conditional If and Its Variants

fn control() {
    let n1 = r#"
    pod: Control Structures
    - Analyze variables and chose directions in which to execute the code
    ---
    pod: Conditionals
    - Programming language commands to handle decisions
    ---"#;
    println!("{n1}");
}

fn conditional_if() {
    let marks: i32 = 95;
    let mut grade: char = 'N';
    if marks >= 90 {
        grade = 'A';
    } else if marks >= 80 {
        grade = 'B';
    } else if marks >= 70 {
        grade = 'C';
    } else if marks >= 60 {
        grade = 'D';
    } else {
        grade = 'F';
    }
    println!("Conditional If: grade: {grade}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_control() {
        control()
    }

    #[test]
    fn run_conditional_if() {
        conditional_if()
    }
}
