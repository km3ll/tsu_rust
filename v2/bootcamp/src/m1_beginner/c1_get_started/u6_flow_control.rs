pub fn flow_control() {
	println!("----------");
	println!("Flow Control");

	let a: i8 = 25;
	println!("a: {a}");

	if a > 30 {
		println!("Bigger than 30");
	} else if a > 20 {
		println!("Bigger than 20");
	} else {
		println!("Smaller or equal to 20")
	}

	// if-else expressions can be used in let statements
	let b: i8 = if a > 18 { 1 } else { -1 };
	println!("b: {b}");

	// loop
	loop {
		println!("loop forever");
		break;
	}

	// labeling loops
	// The name must start with a tick (')
	'outer: loop {
		println!("outer loop");
		loop {
			println!("inner loop");
			break 'outer;
		}
	}

	// return values from loop
	let x: i8 = loop {
		println!("loop returning 5");
		break 5;
	};

	// while-loop
	let mut count: i8 = 1;
	while count <= 5 {
		println!("count: {count}");
		count = count + 1;
	}

	// for-loop
	let numbers: [i8; 4] = [10, 20, 30, 40];
	for element in numbers {
		println!("element: {}", element)
	}
}
