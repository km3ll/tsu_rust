pub fn structs() {
	println!("----------");
	println!("Structs");

	let book: Product = Product {
		name: String::from("Book"),
		price: 10.85,
		in_stock: true,
	};

	println!("Book");
	println!(" name: {}", book.name);
	let price: f32 = book.price;
	println!(" price: {price}");

	println!("Chair");
	let mut chair: Product = Product {
		name: String::from("Chair"),
		price: 28.50,
		in_stock: true,
	};

	chair.in_stock = false;
	println!(" in_stock: {}", chair.in_stock);

	let sales_tax = calculate_tax(&chair);
	println!(" sales_tax: {sales_tax}");
}

struct Product {
	name: String,
	price: f32,
	in_stock: bool,
}

fn calculate_tax(product: &Product) -> f32 {
	product.price * 0.1
}
