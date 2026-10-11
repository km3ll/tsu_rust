//! # Functions

pub fn functions() {
    let n1 = r#"
    ---
	pod: Functions
	- Naming convention is `snake-case`
	- To use the last expression as return type, ommit the semicolon.
	---"#;
    println!("{n1}");
}

pub fn functions_statement(x: u32) {
    let n1 = r#"
    ---
	pod: Statements
	- Instructions that do not return a value as in `println!()`
	---"#;
    println!("{n1}");

    println!("Statement");
    println!(" > statement println!(x): {x}")
}

pub fn functions_expression(x: u32) -> u32 {
    let n1 = r#"
    ---
	pod: Expressions
	- Code that evaluates to a value as in `x * 2`
	---"#;
    println!("{n1}");

    println!("Expression");
    let y: u32 = x * 2;
    println!(" > (y = x * 2) y: {y}");
    y
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_functions() {
        functions();
    }

    #[test]
    fn run_functions_statement() {
        functions_statement(11);
    }

    #[test]
    fn run_functions_expression() {
        functions_expression(11);
    }
}
