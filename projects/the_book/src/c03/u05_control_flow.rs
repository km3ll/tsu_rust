use rand::Rng;

fn flow_if_expression() {
	let n1 = r#"
	pod: If Expressions
	- Allow to branch code depending on conditions
	- Blocks of code associated with the conditions are called 'arms'
	---"#;
	println!("{n1}");

	let x = rand::thread_rng().gen_range(1..=10);
	if x > 5 {
		println!("if expression: x: {x}");
	}

	let y = if x == 5 { "is five" } else { "is not five" };
	println!("if expression: y: {y}");
}

fn flow_loops() {
	let n1 = r#"
	pod: Loops
	- Rust has three kinds of loops: loop, while, and for
	---"#;
	println!("{n1}");
}

fn flow_loop() {
	let n1 = r#"
	pod: Loop
	- 'break' stops executing the loop
	- 'continue' skips over any remaining code and goes to next iteration
	- 'return' always exits the current function
	- A return value can be added after the 'break' expression
	- Loop labels must begin with a single quote (')
	---"#;
	println!("{n1}");
}

fn flow_loop_break() {
	loop {
		let x = rand::thread_rng().gen_range(0..=10);
		println!("Loop random: x: {x}");
		if x >= 5 {
			break;
		}
	}
}

fn flow_loop_return_value() {
	let mut i: u8 = 0;
	let result: u8 = loop {
		i += 1;
		if i == 10 {
			break i * 2;
		}
	};
	println!("Loop result: {result}");
}

fn flow_loop_label() {
	let mut count = 0;
	'counting_up: loop {
		println!("labeled: count: {count}");

		let mut remaining = 10;
		loop {
			println!("labeled: remaining: {remaining}");
			if remaining == 9 {
				break;
			}
			if count == 2 {
				println!("labeled: break");
				break 'counting_up;
			}
			remaining -= 1;
		}

		count += 1;
	}
}

fn flow_while_loop() {
	let mut count = 3;
	while count != 0 {
		println!("while: {count}");
		count -= 1;
	}
	println!("while: liftoff!");
}

fn flow_for_loop() {
	let numbers = [10, 20, 30];
	for n in numbers {
		println!("for: n: {n}")
	}
}

fn flow_for_loop_range() {
	for n in (1..=3).rev() {
		println!("for: {n}");
	}
	println!("for: liftoff!");
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn run_flow_if_expression() {
		flow_if_expression();
	}

	#[test]
	fn run_flow_loops() {
		flow_loops();
	}

	#[test]
	fn run_flow_loop() {
		flow_loop()
	}

	#[test]
	fn run_flow_loop_break() {
		flow_loop_break()
	}

	#[test]
	fn run_flow_loop_return_value() {
		flow_loop_return_value()
	}

	#[test]
	fn run_flow_loop_label() {
		flow_loop_label()
	}

	#[test]
	fn run_flow_while_loop() {
		flow_while_loop()
	}

	#[test]
	fn run_flow_for_loop() {
		flow_for_loop()
	}

	#[test]
	fn run_flow_for_loop_range() {
		flow_for_loop_range()
	}
}
