#[derive(Debug)]
struct RGB(i32, i32, i32);

#[derive(Debug)]
struct CMYK(i32, i32, i32, i32);

#[derive(Debug)]
struct MyStruct;

pub fn tuple_structs_tuples() {
	/**
	 * pod: Tuples
	 * - group together data of different types
	 * - do not define a new type
	 */
	let c1: (i32, i32, i32) = (255, 160, 0);
	let c2: (i32, i32, i32, i32) = (0, 58, 100, 0);
	println!("c1: {:?}", c1);
	println!("c2: {:?}", c2);
}

pub fn tuple_structs_definition() {
	/**
	 * pod: Tuple Structs ensure number and parameter types
	 */
	let c1: RGB = RGB(255, 160, 0);
	let c2: CMYK = CMYK(0, 58, 100, 0);
	println!("c1: {:?}", c1);
	println!("c2: {:?}", c2);
}

pub fn tuple_structs_unit_like() {
	/**
	 * pod: Unit-like Structs
	 * - don't have any fields
	 */
	println!("unit-like: {:?}", MyStruct);
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn run_tuple_structs_tuples() {
		tuple_structs_tuples();
		assert!(true)
	}

	#[test]
	fn run_tuple_structs_definition() {
		tuple_structs_definition();
		assert!(true)
	}

	#[test]
	fn run_tuple_structs_unit_like() {
		tuple_structs_unit_like();
		assert!(true)
	}
}
