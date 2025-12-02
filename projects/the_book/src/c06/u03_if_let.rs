use rand::Rng;

fn if_let_definition() {
	let n1 = r#"
	pod: If-Let
	- A way to handle values that match one pattern while ignoring the rest
	- The code only runs if the value matches the pattern
	- We can include and 'else' with an if-let
	- The let...else syntax allows to stay on the happy path
	---"#;
	println!("{n1}");

	let config_max: Option<u8> = Some(3u8);
	if let Some(max) = config_max {
		println!("If-Let: max: {max}");
	}
}

fn if_let() {
	let mut counter = 0;
	let dice_roll = Some(6);
	if let Some(roll) = dice_roll {
		println!("If-Let: roll: {roll}");
	} else {
		counter += 1;
	}
}

fn if_let_else() {
	let dice_roll = Some(6);
	let Some(roll) = dice_roll else { return };
	println!("If-Else: let-else roll: {roll}");
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
