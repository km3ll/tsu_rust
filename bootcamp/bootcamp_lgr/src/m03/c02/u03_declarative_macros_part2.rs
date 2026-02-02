//! # Declarative Macros Part 2

fn base() {
    let n1 = r#"
	pod:
	-
	---"#;
    println!("{n1}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_() {
        base();
    }
}
