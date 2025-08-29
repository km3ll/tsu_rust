pub fn data_types() {
	println!("----------");
	println!("Data Types");

	println!("Scalar");

	println!("signed integers");
	let si1: i8 = -8;
	let si2: i16 = 16;
	let si3: i32 = -32;
	let si4: i64 = 64;
	let si5: i128 = -128;
	println!("si1: {si1}");
	println!("si2: {si2}");
	println!("si3: {si3}");
	println!("si4: {si4}");
	println!("si5: {si5}");

	println!("floating point");
	let fp1: f32 = 1.0;
	let fp2: f64 = 2.0;
	println!("fp1: {fp1}");
	println!("fp2: {fp2}");

	println!("platform specific");
	let ps1: usize = 1;
	let ps2: isize = 2;
	println!("ps1: {ps1}");
	println!("ps2: {ps2}");

	println!("character, string-slice, string");
	let c1: char = 'c';
	let s2: &str = "Hello";
	let s3: String = String::from("Hello");
	println!("c1: {c1}");
	println!("s2: {s2}");
	println!("s3: {s3}");

	println!("Compound");

	println!("arrays"); // multiple values of same type
	let arr1: [i32; 5] = [1, 2, 3, 4, 5];
	let e0: i32 = arr1[0];
	let e4: i32 = arr1[4];
	println!("arr1: {:?}", arr1);
	println!("e4: {e4}");
	println!("e4: {e4}");

	println!("tuples"); // multiple values of different types
	let tup1: (i32, f64, &str) = (5, 6.0, "7");
	println!("tup1: {:?}", tup1);
	println!("tup1_0: {}", tup1.0);
	println!("tup1_1: {}", tup1.1);
	println!("tup1_2: {}", tup1.2);

	let (te1, te2, te3) = tup1;
	println!("te1: {te1}");
	println!("te2: {te2}");
	println!("te3: {te3}");

	println!("unit type (empty tuple)");
	let unit: () = ();
	println!("unit: {:?}", unit);

	println!("type alias");
	type Age = u8;
	let ta1: Age = 40;
	println!("ta1: {ta1}");
}
