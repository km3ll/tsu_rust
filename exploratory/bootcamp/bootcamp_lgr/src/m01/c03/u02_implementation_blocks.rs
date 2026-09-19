//! # Implementation Blocks

use rand::{Rng, random};

#[derive(Debug)]
struct Product {
    name: String,
    price: f32,
    in_stock: bool,
}

impl Product {
    fn calculate_tax(&self) -> f32 {
        self.price * 0.1
    }

    fn set_price(&mut self, price: f32) {
        self.price = price
    }

    fn buy(self) -> u32 {
        let name = self.name;
        println!("{name} was bought!");
        rand::rng().random()
    }

    fn get_default_sales_tax() -> f32 {
        0.1
    }

    fn new(name: String, price: f32) -> Product {
        Product {
            name: name,
            price: price,
            in_stock: true,
        }
    }
}

pub fn impl_blocks() {
    let n1 = r#"
	pod: Implementation Blocks
	- Add functionality for a given type
	- `self` refers to the instance of product (mutable/immutable)
	---
	pod: Owned Form Of Self
	- Usually done when you want to transform to another type while preventing the caller from using the original
	- At the end of the method, self is dropped
	---
	pod: Associated Functions
	- Called static functions in other languages
	- Don't take 'self' as a parameter
	- `new` is a constructor as associated function
	---"#;
    println!("{n1}");
}

pub fn impl_blocks_immutable_borrow() {
    let book = Product {
        name: String::from("Book"),
        price: 10.85,
        in_stock: true,
    };
    let tax: f32 = book.calculate_tax();
    println!("book: {:?}", book);
    println!("tax : {}", tax);
}

pub fn impl_blocks_mutable_borrow() {
    let mut chair = Product {
        name: String::from("Chair"),
        price: 28.50,
        in_stock: true,
    };
    println!("chair: {:?}", chair);
    chair.set_price(21.75);
    println!("chair: {:?}", chair);
}

pub fn impl_blocks_owned_form() {
    let mut lamp = Product {
        name: String::from("Lamp"),
        price: 9.99,
        in_stock: true,
    };
    let receipt: u32 = lamp.buy();
    println!("receipt: {}", receipt)
}

pub fn impl_blocks_associated_functions() {
    let default_tax: f32 = Product::get_default_sales_tax();
    println!("default tax: {}", default_tax);
}

pub fn impl_blocks_constructor_new() {
    let lamp = Product::new(String::from("Lamp"), 13.95);
    println!("new: {:?}", lamp);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_impl_blocks() {
        impl_blocks();
    }

    #[test]
    fn run_impl_blocks_immutable_borrow() {
        impl_blocks_immutable_borrow();
    }

    #[test]
    fn run_impl_blocks_mutable_borrow() {
        impl_blocks_mutable_borrow();
    }

    #[test]
    fn run_impl_blocks_owned_form() {
        impl_blocks_owned_form();
    }

    #[test]
    fn run_impl_blocks_associated_functions() {
        impl_blocks_associated_functions();
    }

    #[test]
    fn run_impl_blocks_constructor_new() {
        impl_blocks_constructor_new();
    }
}
