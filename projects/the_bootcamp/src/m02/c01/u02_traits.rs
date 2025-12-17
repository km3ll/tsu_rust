#[derive(Debug)]
struct Car {
	year: u16,
}

#[derive(Debug)]
struct Truck {
	year: u16,
}

struct House {}

/**
 * pod: Traits
 * - Rust does not support classical inheritance
 * - Similar to interfaces in Java
 * - Used in implementation blocks
 * - Only functionality can be shared
 */

trait Park {
	fn park(&self);
}

/**
 * pod: Trait's default implementation
 */
trait Paint {
	fn paint(&self, color: &str) {
		println!("painting object: {}", color)
	}
}

impl Paint for Car {}
impl Paint for Truck {}
impl Paint for House {
	fn paint(&self, color: &str) {
		println!("painting house: {}", color)
	}
}

impl Park for Car {
	fn park(&self) {
		println!("parking car!")
	}
}

impl Park for Truck {
	fn park(&self) {
		println!("parking truck!")
	}
}

pub fn traits_impl_block() {
	let car = Car { year: 2012 };
	println!("car: {:?}", car);
	car.park();

	let truck = Truck { year: 2012 };
	println!("truck: {:?}", truck);
	truck.park();
}

pub fn traits_default_impl() {
	let car = Car { year: 2012 };
	println!("car: {:?}", car);
	car.paint("gray");

	let truck = Truck { year: 2012 };
	println!("truck: {:?}", truck);
	truck.paint("black");
}

pub fn traits_override() {
	let house = House {};
	house.paint("white");
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn run_traits_impl_block() {
		traits_impl_block();
	}

	#[test]
	fn run_traits_default_impl() {
		traits_default_impl();
	}

	#[test]
	fn run_traits_override() {
		traits_override();
	}
}
