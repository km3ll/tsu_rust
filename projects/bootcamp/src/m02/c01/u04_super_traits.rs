/**
 * pod: Super traits
 * - A trait can rely on other traits to be implemented
 * - Paint is a super-trait of vehicle
 * - Any type implementing the Vehicle trait must also
 *   implement the Paint trait
 * - More super-traits added with the plus (+) symbol
 */
trait Draw {
	fn draw(&self);
}

/**
 * pod: Traits with methods
 * - functions where their first argument is &self
 */
trait Paint {
	fn paint(&self);
}

/**
 * pod: Traits with associated functions
 */
trait Vehicle: Paint + Draw {
	fn drive(&self);

	fn get_default_color() -> String {
		String::from("black")
	}
}

struct Car {
	year: u16,
}

impl Draw for Car {
	fn draw(&self) {
		println!("drawing car!");
	}
}

impl Paint for Car {
	fn paint(&self) {
		println!("painting car!");
	}
}

impl Vehicle for Car {
	fn drive(&self) {
		println!("driving car!");
	}
}

pub fn super_traits_multiple() {
	let car = Car { year: 2025 };
	car.draw();
	car.paint();
	car.drive();
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn run_super_traits_multiple() {
		super_traits_multiple();
	}
}
