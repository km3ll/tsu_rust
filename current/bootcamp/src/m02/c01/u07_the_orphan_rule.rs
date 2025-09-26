struct Point {
	x: i32,
	y: i32,
}

/**
 * pod: orphan rule
 * - In order to implement a trait on a given type either the trait
 *   or the type must be defined in the current crate
 * - Only traits defined in current crate can be implemented
 *   for arbitrary types
 */
impl PartialEq for Point {
	fn eq(&self, other: &Self) -> bool {
		self.x == other.x && self.y == other.y
	}
}

/**
 * pod: Wrapper alternative
 * - Creating a tuple struct
 */
struct PointWrapper(Point);

impl PartialEq for PointWrapper {
	fn eq(&self, other: &Self) -> bool {
		self.0.x == other.0.x && self.0.y == other.0.y
	}
}

fn orphan_rule() {
	let p1 = PointWrapper(Point { x: 3, y: 1 });
	let p2 = PointWrapper(Point { x: 3, y: 1 });
	let p3 = PointWrapper(Point { x: 8, y: 2 });

	println!("p1 == p2: {}", p1 == p2);
	println!("p1 == p3: {}", p1 == p3);
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn run_orphan_rule() {
		orphan_rule();
	}
}
