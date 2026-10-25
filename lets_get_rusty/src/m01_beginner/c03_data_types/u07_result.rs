//! # Result

fn run(query: String) -> Result<String, String> {
    println!(" > func: run()");
    if query.is_empty() {
        Err(String::from("empty-query"))
    } else {
        Ok(String::from(" Ferris"))
    }
}

fn find_name(query: String) -> Option<String> {
    println!(" > func: find_name()");

    let r1: Result<String, String> = run(query);
    println!(" > func: r1: {r1:?}");

    let r2 = r1.ok();
    println!(" > func: r1.ok(): {r2:?}");
    r2
}

pub fn result_enum() {
    let n1 = r#"
    ---
	pod: Result
	- Defined and loaded in the prelude
	- ok() converts Result into Option

	pod: If Statements
	- Parenthesis can be omitted
	---"#;
    println!("{n1}");

    println!("Result Enum");
    let rs1 = run(String::from(""));
    println!(" > rs1: {rs1:?}");

    let rs2 = run(String::from("query"));
    println!(" > rs2: {rs2:?}");
}

pub fn result_to_option() {
    println!("Result Enum");

    let op1 = find_name(String::from(""));
    println!(" > main: op1: {op1:?}");

    let op2 = find_name(String::from("query"));
    println!(" > main: op2: {op2:?}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_result_enum() {
        result_enum();
    }

    #[test]
    fn run_result_to_option() {
        result_to_option();
    }
}
