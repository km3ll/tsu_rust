#[derive(Debug)]
struct Car {
	year: u16,
}

trait Paint {
	fn paint(&self, color: String) {
		println!("painting object: {}", color)
	}
}

trait Park {
	fn park(&self) {
		println!("parking object")
	}
}

impl Paint for Car {}
impl Park for Car {}

/**
 * pod: Trait Bound
 * - There are three ways to specify bounds
 *   - 1. T: Paint
 *   - 2. impl syntax
 *   - 3. where syntax (multiple)
 * - Can be used as return types
 */
fn paint_red_v1<T: Paint>(object: &T) {
	object.paint("red".to_owned());
}

fn paint_red_v2(object: &impl Paint) {
	object.paint("red".to_owned());
}

fn paint_red_v3<T>(object: &T)
where
	T: Paint + Park,
{
	object.paint("red".to_owned());
}

fn trait_bounds_colon_syntax() {
	let car = Car { year: 2020 };
	paint_red_v1(&car);
}

pub fn trait_bounds_impl_syntax() {
	let car = Car { year: 2022 };
	paint_red_v2(&car);
}

fn trait_bounds_where_syntax() {
	let car = Car { year: 2023 };
	paint_red_v3(&car);
}

fn create_paintable_object() -> impl Paint {
	Car { year: 2021 }
}

fn trait_bounds_return_type() {
	let p1 = create_paintable_object();
	p1.paint("blue".to_owned());
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn run_trait_bounds_colon_syntax() {
		trait_bounds_impl_syntax();
	}

	#[test]
	fn trait_bounds_colon_syntax() {
		trait_bounds_impl_syntax();
	}

	#[test]
	fn run_trait_bounds_where_syntax() {
		trait_bounds_where_syntax();
	}

	#[test]
	fn run_trait_bounds_return_type() {
		trait_bounds_return_type();
	}
}
