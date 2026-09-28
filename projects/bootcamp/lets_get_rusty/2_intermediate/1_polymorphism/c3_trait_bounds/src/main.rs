#![allow(unused)]

struct VehicleInfo {
    make: String,
    model: String,
    year: u16,
}

struct Car {
    info: VehicleInfo,
}

struct House {}

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
    
    let car: Car = Car {
        info: VehicleInfo {
            make: "Toyota".to_owned(),
            model: "Camry".to_owned(),
            year: 2015,
        },
    };

    let house: House = House {};

    let object = create_paintable_object();

    paint_red_one(&car);
    paint_red_one(&house);
    paint_red_one(&object);

    paint_vehicle_red(&car);
    // paint_vehicle_red(&house); Error: does not implement Park trait
    // paint_vehicle_red(&object); Error: does not implement Park trait

}

// 1. There are three ways to specify trait bounds
// A. Using 'T: <trait-name>'
fn paint_red_one<T: Paint>(object: &T) {
    object.paint("red".to_owned());
}

// B. Using the 'impl' syntax
// Object must be a reference to something that implements the Paint trait
fn paint_red_two(object: &impl Paint) {
    object.paint("red".to_owned());
}

// C. Using the 'where' clause
fn paint_red_three<T>(object: &T) where T: Paint {
    object.paint("red".to_owned());
}

// Using two trait bounds
fn paint_vehicle_red<T>(object: &T) where T: Paint + Park {
    object.paint("red".to_owned());
}

// 2. Traits can also be use as return types
// In this example we return something that implements the Paint trait.
// This only works if we're returning one concrete type
fn create_paintable_object() -> impl Paint {
    House {}
}