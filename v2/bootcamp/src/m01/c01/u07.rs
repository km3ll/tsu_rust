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
	// pod: if-else expressions can be used in let statements
	let a: i8 = 25;
	let b: i8 = if a > 18 { 1 } else { -1 };
	println!("b: {b}");
}

pub fn flow_loop() {
	loop {
		// pod: the 'break' statement exits a loop
		println!("loops forever");
		break;
	}
}

pub fn flow_labeling_loops() {
	/***
	 * pod: when labeling loops, their name must start with a tick (')
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
	fn flow_ifelse_runs() {
		flow_ifelse();
		assert!(true)
	}

	#[test]
	fn flow_ifelse_in_let_runs() {
		flow_ifelse_in_let();
		assert!(true)
	}

	#[test]
	fn flow_loop_runs() {
		flow_loop();
		assert!(true)
	}

	#[test]
	fn flow_labeling_loops_runs() {
		flow_labeling_loops();
		assert!(true)
	}

	#[test]
	fn flow_loop_returning_value_runs() {
		flow_loop_returning_value();
		assert!(true)
	}

	#[test]
	fn flow_while_loop_runs() {
		flow_while_loop();
		assert!(true)
	}

	#[test]
	fn flow_for_loop_runs() {
		flow_for_loop();
		assert!(true)
	}
}
