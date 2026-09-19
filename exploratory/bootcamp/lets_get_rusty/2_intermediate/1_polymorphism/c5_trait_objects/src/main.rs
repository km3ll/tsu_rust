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

struct House {}

impl Paint for House {}

// We can use trait balance as a return type if we are returning one concrete type
// Remember that `impl Pain` is syntax sugar for using a generic with a trait bound.
// When using a generic as a return type that generic must be substituted with one
// concrete type at compile time

// Box<dyn Paint>
// Trait objects allow us to define a type which implements a trait without knowing
// what type is at compile time. Trait objects are define with dyn = dynamic dispatch
// keyword and must be behind some type of pointer, for instance `Box`

// The advantage of dynamic dispatch is flexibility. 
// The disadvantage is a runtime performance cost

// Static dispatch
// When the compiler knows which concrete methods to call at compile time

fn create_paintable_object(isVehicle: bool) -> Box<dyn Paint> {
    if isVehicle {
        Box::new(Car {})
    } else {
        Box::new(House {})
    }
    
}

fn paint_blue(object: &dyn Paint) {
    object.paint("blue".to_owned());
}


fn main() {
    let car: Car = Car {};
    paint_vehicle_red(&car);

    let object: Box<dyn Paint> = create_paintable_object(true);
    paint_blue(object.as_ref());

    
    // Example of a collection of object that implement a certain trait
    let house = House {};
    let paintable_objects: Vec<&dyn Paint> = vec![&car, &house];

}