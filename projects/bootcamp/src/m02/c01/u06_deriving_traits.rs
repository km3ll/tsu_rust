#[derive(Debug, PartialEq)]
struct Point {
	x: i32,
	y: i32,
}

/**
 * pod {:?} syntax
 * - To print a type with debug formatting
 * - Only works if the type implements the Debug trait
 * - Can be derived using #[derive(Debug)]
 */
fn deriving_traits_debug() {
	let p1 = Point { x: 3, y: 1 };
	println!("p1: {:?}", p1)
}

/**
 * pod: PartialEq
 * - Can be derived to compare with == operator
 */
fn deriving_traits_partial_eq() {
	let p1 = Point { x: 3, y: 1 };
	let p2 = Point { x: 3, y: 1 };
	let res = p2 == p2;

	println!("p1 == p2: {}", res);
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn run_deriving_traits_debug() {
		deriving_traits_debug();
	}

	#[test]
	fn run_deriving_traits_partial_eq() {
		deriving_traits_partial_eq();
	}
}
