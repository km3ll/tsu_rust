//! # Option

use rand::Rng;

fn find_name(id: u32) -> Option<String> {
    println!(" > func: find_name()");
    let name = String::from("Ferris");
    if (1..=10).contains(&id) {
        Some(name)
    } else {
        None
    }
}

pub fn option_enum() {
    let n1 = r#"
    ---
	pod: Option
	- Defined and loaded in the prelude
	- if-let syntax works when you only care about one variant
	---"#;
    println!("{n1}");

    println!("Option");
    let id: u32 = rand::rng().random_range(1..20);
    println!(" > main: id: {id}");

    let op: Option<String> = find_name(id);
    println!(" > main: match:");
    match op {
        Some(name) => println!(" > name: {name}"),
        None => println!(" > not found!"),
    }
}

pub fn option_if_let_syntax() {
    println!("Option If-Let Syntax");

    let op1 = find_name(1);
    println!(" > main: op1: {op1:?}");

    if let Some(name) = op1 {
        println!(" > main: if-let-some: {name}")
    }

    let op2 = find_name(20);
    println!(" > main: op2: {op2:?}");

    if let None = op2 {
        println!(" > main: if-let-none: not found!")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_option_enum() {
        option_enum();
    }

    #[test]
    fn run_option_if_let_syntax() {
        option_if_let_syntax();
    }
}
