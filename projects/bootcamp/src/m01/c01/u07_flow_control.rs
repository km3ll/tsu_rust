#[allow(unused)]
use rand::Rng;

pub fn flow_ifelse() {
	let a1 = rand::rng().random_range(-40..40);
	if a1 > 30 {
		println!("ifelse: Bigger than 30");
	} else if 1 > 20 {
		println!("ifelse: Bigger than 20");
	} else {
		println!("ifelse: Smaller or equal to 20")
	}
}

pub fn flow_ifelse_in_let() {
	let n1 = r#"
	pod: If-Else Expressions
	- Can be used in let statements
	---"#;
	println!("{n1}");

	let a1: i8 = 25;
	let b1: i8 = if a1 > 18 { 1 } else { -1 };
	println!("let expression b1: {b1}")
}

pub fn flow_loop() {
	let n1 = r#"
	pod: Loops
	- loop
	  - inner / outer
	  - break
	  - break 'name (name starts with tick ('))
	  - brear value
	- while
	- for
	---"#;
	println!("{n1}");

	loop {
		println!("loop: break");
		break;
	}
}

pub fn flow_labeling_loops() {
	println!("outer loop");
	'name: loop {
		println!("inner loop");
		loop {
			println!("loop: break 'name");
			break 'name;
		}
	}
}

pub fn flow_loop_returning_value() {
	let x: i8 = loop {
		println!("loop: break 5");
		break 5;
	};
}

pub fn flow_while_loop() {
	let mut i: i8 = 1;
	while i <= 5 {
		println!("while-loop i: {i}");
		i = i + 1;
	}
}

pub fn flow_for_loop() {
	let elements: [i8; 4] = [10, 20, 30, 40];
	for e in elements {
		println!("for-loop e: {e}")
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn run_flow_ifelse() {
		flow_ifelse();
	}

	#[test]
	fn run_flow_ifelse_in_let() {
		flow_ifelse_in_let();
	}

	#[test]
	fn run_flow_loop() {
		flow_loop();
	}

	#[test]
	fn run_flow_labeling_loops() {
		flow_labeling_loops();
	}

	#[test]
	fn run_flow_loop_returning_value() {
		flow_loop_returning_value();
	}

	#[test]
	fn run_flow_while_loop() {
		flow_while_loop();
	}

	#[test]
	fn run_flow_for_loop() {
		flow_for_loop();
	}
}
