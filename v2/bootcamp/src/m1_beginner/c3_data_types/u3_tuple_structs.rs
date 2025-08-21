pub fn tuple_structs() {
	println!("----------");
	println!("Tuple Structs");

	/**
	 * Tuples
	 * - group together data of different types
	 * - do not define a new type
	 */
	println!("Tuples");
	let rgb_color: (i32, i32, i32) = (255, 160, 0);
	let cmyk_color: (i32, i32, i32, i32) = (0, 58, 100, 0);
	println!(" > rgb: {:?}", rgb_color);
	println!(" > cmyk: {:?}", cmyk_color);

	println!("Structs");
	struct RGB(i32, i32, i32);
	struct CMYK(i32, i32, i32, i32);

	let color1: RGB = RGB(255, 160, 0);
	let color2: CMYK = CMYK(0, 58, 100, 0);
	println!(" > color1: RGB(255, 160, 0)");
	println!(" > color2: CMYK(0, 58, 100, 0)");

	println!("Unit-like Structs");
	struct MyStruct;
	println!(" > my_struct: MyStruct");
}
