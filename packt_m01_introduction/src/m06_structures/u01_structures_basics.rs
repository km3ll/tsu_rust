//! # Structures Basics

#[derive(Debug)]
struct Person {
    name: String,
    age: i32,
    salary: i32,
}

impl Person {
    fn new(name: String, age: i32, salary: i32) -> Self {
        Person { name, age, salary }
    }

    fn compute_taxes(&self) -> f32 {
        (self.salary as f32 / 3.) * 0.5
    }
}

fn structures() {
    let n1 = r#"
    pod: Struct
    - Data type that groups items of different types
    - The naming convention is CamelCase
    - It is not necesary to match the order of field definitions
    - The initialization function is typically named `new()`
    - Keyword `Self` refers to the struct name itself
    - Can be initialized from other struct with the `..` notation
    ---"#;
    println!("{n1}");

    let p1 = Person {
        age: 30,
        name: String::from("Ferris"),
        salary: 40_000,
    };
    println!("Structs: p1: {p1:?}");

    let taxes1 = p1.compute_taxes();
    println!(" > taxes1: {taxes1}");

    let taxes2 = Person::compute_taxes(&p1);
    println!(" > taxes2: {taxes2}");
}

fn structures_impl_blocks() {
    let p2 = Person::new(String::from("John Wick"), 30, 200_000);
    println!("Structs: p2: {p2:?}");

    let p3 = Person {
        name: String::from("Ferris, The Crab"),
        ..p2
    };
    println!(" > initialized p3: {p3:?}");
}

#[derive(Debug)]
struct Coordinate(i32, i32);

impl Coordinate {
    fn greater(&self) -> i32 {
        if self.0 >= self.1 { self.0 } else { self.1 }
    }
}

fn tuple_structs() {
    let n1 = r#"
    pod: Tupple Structs
    - A named tuple
    - Can have implementation blocks
    ---"#;
    println!("{n1}");

    let c1 = Coordinate(0, 10);
    println!("Tuple Structs: c1: {c1:?}");

    let greater = c1.greater();
    println!(" > greater: {greater}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_structures() {
        structures()
    }

    #[test]
    fn run_structures_impl_blocks() {
        structures_impl_blocks()
    }

    #[test]
    fn run_tuple_structs() {
        tuple_structs()
    }
}
