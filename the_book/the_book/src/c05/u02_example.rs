//! # An Example Program Using Structs

fn area_v1(width: u32, height: u32) -> u32 {
	width * height
}

fn area_v2(dimensions: (u32, u32)) -> u32 {
	dimensions.0 * dimensions.1
}

#[derive(Debug)]
struct Rectangle {
	width: u32,
	height: u32,
}

fn area_v3(rectangle: &Rectangle) -> u32 {
	rectangle.width * rectangle.height
}

fn example_v1() {
	let width: u32 = 30;
	let height: u32 = 50;
	let area = area_v1(width, height);
	println!("Structs: area v1: {area}");
}

fn example_v2() {
	let dimensions: (u32, u32) = (30, 50);
	let area: u32 = area_v2(dimensions);
	println!("Structs: area v2: {area}");
}

fn example_v3() {
	let r1 = Rectangle {
		width: 30,
		height: 50,
	};
	let area = area_v3(&r1);
	println!("Structs: area v3: {area}");
}

fn example_using_structs() {
	let n1 = r#"
	pod: Example Using Structs
	- v1. It's not clear anywhere in our program that the parameters are related
	- v2. Tuples don't name their elements, so we have to index the parts of the tuple
	- v3. We want to borrow the struct, so that main retains its ownership
	- Accessing fields of a borrowed struct does not move field values
	---"#;
	println!("{n1}");
}

fn example_derived_traits() {
	let n1 = r#"
	pod: Derived Trait: Display
	- User consumption
	- Curly brackets {} tell println!() to use Display formatting
	- Structs don't have a provided implementation of Display
	  - Do you want commas or not?
	  - Do you want to print the curly brackets?
	  - Should all the fields be shown?
	---
	pod: Derived Trait: Debug
	- Developer consumption
	- Specifiers ':?' or ':#?' (pretty-print)
	---
	pod: Outer Attribute: #[derive()]
	- To explicitly opt in to print out debugging information
	---"#;
	println!("{n1}");

	let r1 = Rectangle {
		width: 50,
		height: 70,
	};
	println!("Structs derived trait r1: {r1:#?}");
}

fn example_dbg_macro() {
	let n1 = r#"
	pod: Macro: dbg!
	- Takes ownership of an expression (println! takes a reference)
	- Prints the file and line number of where that macro call occurs
	- Prints also the resultant value of that expression
	- Returns ownership of the value
	---"#;
	println!("{n1}");

	let r1 = Rectangle {
		width: 50,
		height: 70,
	};
	let area = dbg!(50 * r1.height);
	dbg!(&area);
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn run_example_v1() {
		example_v1();
	}

	#[test]
	fn run_example_v2() {
		example_v2();
	}

	#[test]
	fn run_example_v3() {
		example_v3();
	}

	#[test]
	fn run_example_using_structs() {
		example_using_structs();
	}

	#[test]
	fn run_example_derived_traits() {
		example_derived_traits();
	}

	#[test]
	fn run_example_dbg_macro() {
		example_dbg_macro();
	}
}
