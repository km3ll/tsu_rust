use std::cmp::PartialOrd;

fn largest_v1() {
	let numbers = vec![34, 50, 35, 100, 65, 80];
	let mut largest = &numbers[0];
	for number in &numbers {
		if number > largest {
			largest = number;
		}
	}
	println!("Generics: function v1: {largest}");
}

fn largest_v2(numbers: &[i32]) {
	let mut largest = &numbers[0];
	for number in numbers {
		if number > largest {
			largest = number;
		}
	}
	println!("Generics: function v2: {largest}");
}

fn largest_v3<T: PartialOrd>(list: &[T]) -> &T {
	let mut largest: &T = &list[0];
	for item in list {
		if item > largest {
			largest = item;
		}
	}
	largest
}

#[derive(Debug)]
struct PointV1<T> {
	x: T,
	y: T,
}

impl<T> PointV1<T> {
	fn x(&self) -> &T {
		&self.x
	}
}

#[derive(Debug)]
struct PointV2<T, U> {
	x: T,
	y: U,
}

fn generics_definition() {
	let n1 = r#"
	pod: Generics
	- Abstract stand-ins for concrete types or other properties
	- Allow to replace specific types with a placeholder that represent multiple types
	---"#;
	println!("{n1}");
}

fn generics_largest_v1() {
	largest_v1();
}

fn generics_largest_v2() {
	let n1 = vec![34, 50, 35, 100, 65, 80];
	largest_v2(&n1);
	let n2 = vec![102, 34, 6000, 89, 54, 2, 43, 8];
	largest_v2(&n2);
}

fn generics_in_functions() {
	let n1 = r#"
	pod: Trait: PartialOrd
	- We want to compare values of type T. We can only use types whose values can be ordered
	- The standard library implementes this trait on both i32 and char
	---"#;
	println!("{n1}");

	let numbers = vec![34, 50, 25, 100, 65, 98];
	let r1 = largest_v3(&numbers);
	println!("Generics: function v3: {r1}");

	let chars = vec!['h', 'a', 'k', 'u'];
	let r2 = largest_v3(&chars);
	println!("Generics: function v3: {r2}");
}

fn generics_in_structs() {
	let p1 = PointV1 { x: 5, y: 10 };
	println!("Generics: structs p1: {p1:?}");

	let p2 = PointV1 { x: 1.1, y: 4.3 };
	println!("Generics: structs p2: {p2:?}");

	let p3 = PointV2 { x: 6, y: 7.8 };
	println!("Generics: structs p3: {p3:?}");
}

fn generics_in_enums() {
	let op1 = Some("Hello");
	println!("Generics: enums op1: {op1:?}");
}

fn generics_in_methods() {
	let p1 = PointV1 { x: 5, y: 10 };
	println!("Generics: methods p1.x: {}", p1.x());
}

fn generics_() {}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn run_generics_definition() {
		generics_definition();
	}

	#[test]
	fn run_generics_largest_v1() {
		generics_largest_v1();
	}

	#[test]
	fn run_generics_largest_v2() {
		generics_largest_v2();
	}

	#[test]
	fn run_generics_in_functions() {
		generics_in_functions();
	}

	#[test]
	fn run_generics_in_structs() {
		generics_in_structs();
	}

	#[test]
	fn run_generics_in_enums() {
		generics_in_enums();
	}

	#[test]
	fn run_generics_in_methods() {
		generics_in_methods();
	}

	#[test]
	fn run_() {
		generics_();
	}
}
