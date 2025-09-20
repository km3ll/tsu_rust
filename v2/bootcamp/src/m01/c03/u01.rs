/**
 * pod: Structs allow to group related data together
 */
#[derive(Debug)]
struct Product {
	name: String,
	price: f32,
	in_stock: bool,
}

fn calculate_tax(product: &Product) -> f32 {
	product.price * 0.1
}

pub fn structs_instance() {
	let book = Product {
		name: String::from("Book"),
		price: 10.85,
		in_stock: true,
	};
	println!("book: {:?}", book)
}

pub fn structs_mutable() {
	let mut chair = Product {
		name: String::from("Chair"),
		price: 28.50,
		in_stock: true,
	};
	chair.in_stock = false;
	println!("chair: {:?}", chair)
}

pub fn structs_function() {
	let lamp = Product {
		name: String::from("Lamp"),
		price: 12.99,
		in_stock: true,
	};
	let tax = calculate_tax(&lamp);
	println!("lamp: {:?}", lamp);
	println!("tax: {}", tax);
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn run_structs_instance() {
		structs_instance();
	}

	#[test]
	fn run_structs_mutable() {
		structs_mutable();
	}

	#[test]
	fn run_structs_function() {
		structs_function();
	}
}
