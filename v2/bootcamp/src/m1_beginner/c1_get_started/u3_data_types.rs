pub fn data_types() {
	println!("----------");
	println!("Data Types");

	println!("Scalar");

	println!("platform specific");
	

	println!("character, string-slice, string");


	println!("Compound");

	println!("arrays"); // multiple values of same type


	println!("tuples"); // multiple values of different types
	

	println!("unit type (empty tuple)");
	let unit: () = ();
	println!("unit: {:?}", unit);

	println!("type alias");
	type Age = u8;
	let ta1: Age = 40;
	println!("ta1: {ta1}");
}
