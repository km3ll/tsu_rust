#![allow(unused)]

// A trait can rely on other traits being implemented.
// The traits relied on are called super traits.
trait Paint {
    // Default implementation that could be overridden
    fn paint(&self, color: String) {
        println!("Printing object: {}", color);
    }
}

// Paint is now a super trait of Vehicle.
// Any type implementing the Vehicle trait must also 
// implement the Paint trait

// trait Vehicle: Paint + Another {
trait Vehicle: Paint {
    fn park(&self);

    // Traits can also contain associated functions
    fn get_default_color() -> String {
        "black".to_owned()
    }

}

// Using a super trait
fn paint_vehicle_red<T>(object: &T) where T: Vehicle {
    object.paint("red".to_owned());
}


struct Car {}

impl Vehicle for Car {
    fn park(&self) {
        println!("Parking car!");
    }
}

// The only method that has to be implemented has a default implementation
impl Paint for Car {}

fn main() {
    let car: Car = Car {};
    paint_vehicle_red(&car);
}