//! # Traits

#[derive(Debug)]
struct Car {
    year: u16,
}

#[derive(Debug)]
struct Truck {
    year: u16,
}

#[derive(Debug)]
struct House {}

trait Park {
    fn park(&self);
}

trait Paint {
    fn paint(&self, color: &str) {
        println!(" impl<Trait>: paint color: {color}");
    }
}

impl Paint for Car {}
impl Paint for Truck {}
impl Paint for House {
    fn paint(&self, color: &str) {
        println!(" impl<House>: paint({color})")
    }
}

impl Park for Car {
    fn park(&self) {
        println!(" > impl<Car> park()")
    }
}

impl Park for Truck {
    fn park(&self) {
        println!(" > impl<Truck> park()")
    }
}

pub fn traits() {
    let n1 = r#"
    ---
    pod: Traits
    - Rust does not support classical inheritance
    - Similar to interfaces in Java
    - Used in implementation blocks
    - Only functionality can be shared

    pod: Trait's Default Implementation
    - Called if not overridden
    ---"#;
    println!("{n1}");
}

pub fn traits_impl_block() {
    println!("Traits & Impl Blocks");

    let car = Car { year: 2012 };
    println!(" > main: car: {car:?}");
    car.park();

    let truck = Truck { year: 2012 };
    println!(" > main: truck: {truck:?}");
    truck.park();
}

pub fn traits_default_impl() {
    println!("Traits & Default Impl");
    let car = Car { year: 2012 };
    println!(" > main: car: {car:?}");
    car.paint("gray");

    let truck = Truck { year: 2012 };
    println!(" > main: truck: {truck:?}");
    truck.paint("black");
}

pub fn traits_override() {
    println!("Traits & Impl Override");
    let house = House {};
    println!(" > main: house: {house:?}");
    house.paint("white");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_traits() {
        traits()
    }

    #[test]
    fn run_traits_impl_block() {
        traits_impl_block();
    }

    #[test]
    fn run_traits_default_impl() {
        traits_default_impl();
    }

    #[test]
    fn run_traits_override() {
        traits_override();
    }
}
