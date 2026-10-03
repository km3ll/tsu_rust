//! # Concise Control Flow with if let and let else

use rand::Rng;

fn if_let_definition() {
	let n1 = r#"
	---
	pod: If-Let
	- A way to handle values that match one pattern while ignoring the rest
	- The code only runs if the value matches the pattern
	- We can include and `else` with an `if-let`
	- The `let...else` syntax allows to stay on the happy path
	---"#;
	println!("{n1}");

	println!("if-let");
	let config_max: Option<u8> = Some(3u8);
	if let Some(max) = config_max {
		println!(" > max: {max}");
	}
}

fn if_let() {
	println!("if-let");
	let mut counter = 0;
	let dice_roll = Some(6);
	if let Some(roll) = dice_roll {
		println!(" > if-let roll: {roll}");
	} else {
		println!(" > else counter");
		counter += 1;
	}
}

fn if_let_else() {
	println!("if-let");
	let dice_roll = Some(6);
	let Some(roll) = dice_roll else { return };
	println!(" > dice roll: {roll}");
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn run_if_let_definition() {
		if_let_definition();
	}

	#[test]
	fn run_if_let() {
		if_let();
	}

	#[test]
	fn run_if_let_else() {
		if_let_else();
	}
}
