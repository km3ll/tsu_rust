use uuid::Uuid;

pub fn implementation_blocks() {
	println!("----------");
	println!("Implementation Blocks");

	let mut book: Product = Product {
		name: String::from("Book"),
		price: 10.85,
		in_stock: true,
	};

	println!("Book");

	// Immutable borrow
	let tax1: f32 = book.calculate_tax();
	println!(" tax: {tax1}");

	// Mutable borrow
	book.set_price(5.0);
	println!(" price: {}", book.price);

	// Owned form
	let receipt: String = book.buy();
	println!(" receipt: {receipt}");
	//book.set_price(2.0); error: borrow of moved value

	println!("Associated Functions");
	let tax2: f32 = Product::get_default_tax();
	println!(" default tax: {tax2}");

	println!("Constructor");
	let chair = Product::new(String::from("Chair"), 21.99);
	println!(" name: {}", chair.name);
	println!(" in_stock: {}", chair.in_stock);
}

struct Product {
	name: String,
	price: f32,
	in_stock: bool,
}

/**
 * Implementation blocks
 * - Method on product instances
 * - 'self' is a keyword that refers to the instance of
 *   product this function is called on.
 */
impl Product {
	/**
	 * Associated Functions
	 * - Sometimes called 'static functions/methods'
	 * - Associated with a type. However, they do not work on
	 *   instances of that type.
	 * - Don't take 'self' as parameter
	 */
	fn get_default_tax() -> f32 {
		0.1
	}

	/**
	 * Constructor: 'new'
	 */
	fn new(name: String, price: f32) -> Product {
		Product {
			name: name,
			price: price,
			in_stock: true,
		}
	}

	/**
	 * Three forms of self a method can take
	 * - immutable borrow to self
	 * - mutable borrow to self
	 * - owned form of self
	 */

	// Immutable borrow
	fn calculate_tax(&self) -> f32 {
		self.price * 0.1
	}

	// Immutable borrow
	fn set_price(&mut self, price: f32) {
		self.price = price;
	}

	/**
	 * Owned form
	 * - Usually done when you want to transform one type to another type,
	 *   while also preventing the caller from using the original instance.
	 * - Ownership of the instance will be passed into this method. So, at
	 *   the end of the method, self will be dropped and the instance will
	 *   no longer be valid.
	 */
	fn buy(self) -> String {
		let name: String = self.name;
		println!(" {name} was bought!");
		Uuid::new_v4().to_string()
	} // book is dropped
}
