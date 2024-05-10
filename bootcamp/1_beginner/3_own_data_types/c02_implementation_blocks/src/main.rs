#![allow(unused)]
use rand::Rng;

// struct
struct Product {
    name: String,
    price: f32,
    in_stock: bool
}

// Implementation block
// &self: reference to the instance this function is called on
impl Product {

    // Methods 

    // Methods are declared with their first argument as &self
    // There are 3 forms of &self a method can take

    // 1. Immutable borrow to self
    fn calculate_sales_tax(&self) -> f32 {
        &self.price * 0.1
    }

    // 2. Mutable borrow to self
    fn set_price(&mut self, price: f32) -> () {
        self.price = price;
    }

    // 3. Owned form of self
    // It is used when you want to transform one type into another type while
    // also preventing the caller from using the original instance
    fn buy(self) -> () {
        // ownership of the instance is passed to this method
        let name = self.name;
        let receipt: i32 = rand::thread_rng().gen_range(1000..=9999);
        println!("Thank you for your purchase of {}. Your receipt number is: {}", name, receipt);
    } // &self is dropped here and the instance is no longer valid

    // Associated Functions

    // a.k.a. static methods, static functions
    // Associated functions do not take &self as a parameter. Also, they are not called using
    // dot syntax.
    fn get_default_sales_tax() -> f32 {
        0.05
    }

    // A common pattern is to have an associated function called new which acts as a Constructor
    fn new(name: String, price: f32) -> Product {
        // This is the true constructor
        Product {
            name: name,
            price: price,
            in_stock: true
        }
    }

}

fn main() {
    let mut book = Product {
        name: String::from("Fahrenheit 451"),
        price: 29.74,
        in_stock: true
    };

    println!("{}", book.name);
    
    book.set_price(31.50);
    println!(" > price: {}", book.price);

    let sales_tax = book.calculate_sales_tax();
    println!(" > tax: {}", sales_tax);

    let default_sales_tax: f32 = Product::get_default_sales_tax();
    println!(" > default tax: {}", default_sales_tax);

    book.buy();

    /*
    Error: value borrowed after move
    book.set_price(12.99); 
     */

    let book2 = Product::new(
        String::from("The Illustrated Man"),
        14.99
    );
    println!("New book: {}", book2.name);
}

