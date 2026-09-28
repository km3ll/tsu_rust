//! # Program Outputs and Comments

fn outputs_definition() {
    let n1 = r#"
    ---
    macro: `println!()`
    - Prints to the standard output, with a newline.
    - Escape characters: `\n` new line, `\t` tab, `\r carriage return`
    - Allows multi-line messages and output arguments
    - Positional arguments: `{0} {1}`
    - Named arguments `{one} {other}`
    - Math arguments `{}, 25 + 10`
    - Grouped arguments `{:?}, (x, y)`

    macro: `print!()`
    - Prints to the standard output.
    - A newline is not printed at the end of the message.
    ---"#;

    println!("{n1}");
    println!("Outputs: println! moves to next line");
    print!("Outputs: print! ");
    print!("doesn't");
    println!();
    println!(
        "Outputs: message
	     printed as multi-line"
    );
}

fn outputs_escape_seq() {
    println!("\n\nEscape: new line");
    println!("\tEscape: tab");
    println!("Erased part \rEscape: carriage return");
    println!("Escape: \'single quote\'");
    println!("Escape: \"double quotes\"");
    println!("Escape: back slash \\");
}

fn outputs_arguments() {
    println!("Outputs: {1} {0}", "arguments", "positional");
    println!("Outputs: {one} {other}", one = "named", other = "arguments");
    println!("Outputs: math arguments: 25 + 10 = {}", 25 + 10);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_outputs_definition() {
        outputs_definition();
    }

    #[test]
    fn run_outputs_escape_seq() {
        outputs_escape_seq();
    }

    #[test]
    fn run_outputs_arguments() {
        outputs_arguments();
    }
}
