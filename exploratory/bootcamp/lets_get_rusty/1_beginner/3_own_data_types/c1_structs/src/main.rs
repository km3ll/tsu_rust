#![allow(unused)]

struct Product {
    name: String,
    price: f32,
    in_stock: bool
}

fn main() {

    let book1: Product = Product {
        name: String::from("Fahrenheit 451"),
        price: 28.85,
        in_stock: true
    };

    let mut book2: Product = Product {
        name: String::from("The Illustrated Man"),
        price: 14.99,
        in_stock: true
    };
    book2.in_stock = false;

    let sales_tax: f32 = calculate_sales_tax(&book1);
    println!("Sales tax for book '{}' is {}", book1.name, sales_tax);

}

fn calculate_sales_tax(product: &Product) -> f32 {
    product.price * 0.1 // 10%
}