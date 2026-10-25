//! # Structs

#[derive(Debug)]
struct Product {
    name: String,
    price: f32,
    in_stock: bool,
}

fn calculate_tax(product: &Product) -> f32 {
    println!(" > func: calculate_tax()");
    product.price * 0.1
}

pub fn structs() {
    let n1 = r#"
    ---
	pod: Structs
	- Allow to group related data together
	---"#;
    println!("{n1}");
}

pub fn structs_instance() {
    println!("Structs");
    let book = Product {
        name: String::from("Book"),
        price: 10.85,
        in_stock: true,
    };
    println!(" > s1: {book:?}")
}

pub fn structs_mutable() {
    println!("Structs");
    let mut chair = Product {
        name: String::from("Chair"),
        price: 28.50,
        in_stock: true,
    };
    chair.in_stock = false;
    println!(" > mut s2: {chair:?}")
}

pub fn structs_function() {
    println!("Structs");
    let lamp = Product {
        name: String::from("Lamp"),
        price: 12.99,
        in_stock: true,
    };

    println!(" > main: {lamp:?}");
    let tax = calculate_tax(&lamp);
    println!(" > main: tax: {tax}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_structs() {
        structs();
    }

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
