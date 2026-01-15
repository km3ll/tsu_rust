fn outputs_definition() {
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
