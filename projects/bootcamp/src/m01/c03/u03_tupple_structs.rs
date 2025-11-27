#[derive(Debug)]
struct RGB(i32, i32, i32);

#[derive(Debug)]
struct CMYK(i32, i32, i32, i32);

#[derive(Debug)]
struct MyStruct;

pub fn tuple_structs_tuples() {
	let n1 = r#"
	pod: Tuples
	- Group together data of different types
	- Do not define a new type
	---"#;
	println!("{n1}");

	let c1: (i32, i32, i32) = (255, 160, 0);
	let c2: (i32, i32, i32, i32) = (0, 58, 100, 0);
	println!("c1: {:?}", c1);
	println!("c2: {:?}", c2);
}

pub fn tuple_structs_definition() {
	let n1 = r#"
	pod: Tuple Structs
	- Ensure number and parameter types
	---"#;
	println!("{n1}");

	let c1: RGB = RGB(255, 160, 0);
	let c2: CMYK = CMYK(0, 58, 100, 0);
	println!("c1: {:?}", c1);
	println!("c2: {:?}", c2);
}

pub fn tuple_structs_unit_like() {
	let n1 = r#"
	pod: Unit-Like Structs
	- Don't have any fields
	---"#;
	println!("{n1}");

	println!("unit-like: {:?}", MyStruct);
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn run_tuple_structs_tuples() {
		tuple_structs_tuples();
	}

	#[test]
	fn run_tuple_structs_definition() {
		tuple_structs_definition();
	}

	#[test]
	fn run_tuple_structs_unit_like() {
		tuple_structs_unit_like();
	}
}
