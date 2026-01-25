#![allow(unused)]

struct VehicleInfo {
    make: String,
    model: String,
    year: u16,
}

struct Car {
    info: VehicleInfo,
}

struct House { 
    address: String,
}

trait Park {
    fn park(&self);
}

impl Park for Car {
    fn park(&self) {
        println!("Parking car!");
    }
}

trait Paint {
    // Default implementation that could be overridden
    fn paint(&self, color: String) {
        println!("Printing object: {}", color);
    }
}

// The only method that has to be implemented has a default implementation
impl Paint for Car {}

impl Paint for House {
    // Overriding th default implementation
    fn paint(&self, color: String) {
        println!("Painting house");
    }
}

fn main() {
    println!("Hello, world!");
}