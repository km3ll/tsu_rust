pub fn ownership_part2() {
	println!("----------");
	println!("Ownership Part 2");

	/**
	 * Stack-Only data types
	 * - When we call print_integer the value of z is cloned into p2
	 */
	let z: i32 = 30;
	print_integer(z);
}

fn print_string(p1: String) {
	println!("{p1}");
} // p1 is dropped

fn generate_string() -> String {
	String::from("Ferris")
}

fn add_to_string(mut p1: String) -> String {
	//p1.push_str(" is awesome!"); error: cannot borrow 'pa' as mutable
	p1.push_str(" is awesome!");
	p1
}

fn print_integer(p2: i32) {
	println!("p2: {p2}");
}
