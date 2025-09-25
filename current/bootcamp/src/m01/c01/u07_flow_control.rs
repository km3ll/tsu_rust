pub fn flow_ifelse() {
	let a: i8 = 25;
	if a > 30 {
		println!("Bigger than 30");
	} else if a > 20 {
		println!("Bigger than 20");
	} else {
		println!("Smaller or equal to 20")
	}
}

pub fn flow_ifelse_in_let() {
	/**
	 * pod: if-else expressions
	 * - can be used in let statements
	 */
	let a: i8 = 25;
	let b: i8 = if a > 18 { 1 } else { -1 };
	println!("b: {b}");
}

pub fn flow_loop() {
	loop {
		/**
		 * pod: 'break' statement 
		 * - exits a loop
		 */
		println!("loops forever");
		break;
	}
}

pub fn flow_labeling_loops() {
	/***
	 * pod: Labeling loops
	 * - The name must start with a tick (')
	 */
	'outer: loop {
		println!("outer loop");
		loop {
			println!("inner loop");
			break 'outer;
		}
	}
}

pub fn flow_loop_returning_value() {
	let x: i8 = loop {
		println!("loop returning 5");
		break 5;
	};
	println!("Value from loop: {}", x)
}

pub fn flow_while_loop() {
	let mut count: i8 = 1;
	while count <= 5 {
		println!("count: {count}");
		count = count + 1;
	}
}

pub fn flow_for_loop() {
	let numbers: [i8; 4] = [10, 20, 30, 40];
	for element in numbers {
		println!("element: {}", element)
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
