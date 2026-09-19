//! # Trait Objects

trait Paint {
    fn paint(&self);
}

struct Car {}
struct House {}

impl Paint for Car {
    fn paint(&self) {
        println!("painting car");
    }
}

impl Paint for House {
    fn paint(&self) {
        println!("painting house");
    }
}

fn trait_objects() {
    let n1 = r#"
    pod: Trait Object
    - Defined with the `dyn` keyword, which stands for `dynamic dispatch`
    - Must be behind some kind of pointer, like Box smart pointer, which points to something allocated on the heap
    - Trait returns multiple concrete types
    - When using a generic as return type, that generic must be substituted by one concrete type at compile time
    - The paint method of the returned object will be determined at runtime
    ---
    pod: Trait As Parameter
    - Function accepts any reference that implements the Paint trait
    - `fn function(object: &dyn Paint) {}`
    ---
    pod: Trait As Return Type
    - Works only for one concrete type
    - `fn function() -> impl Paint {}`
    ---"#;
    println!("{n1}");
}

fn paint_object(object: &dyn Paint) {
    object.paint();
}

fn create_paintable_object_v1() -> impl Paint {
    House {}
}

fn create_paintable_object_v2(is_car: bool) -> Box<dyn Paint> {
    if is_car {
        Box::new(Car {})
    } else {
        Box::new(House {})
    }
}

pub fn trait_objects_one_concrete_type() {
    let obj = create_paintable_object_v1();
    obj.paint();
}

pub fn trait_objects_static_dynamic() {
    let n1 = r#"
    pod: Static Dispatch
    - The compiler know which concrete methods to call at compile time
    ---
    pod: Dynamic Dispatch
    - The compiler cannot figure out which concrete methods to call
    - Adds a little bit of code to find that out at runtime
    ---"#;
    println!("{n1}");
    println!("static vs dynamic dispatch");
}

pub fn trait_objects_dynamic_types() {
    let car: Box<dyn Paint> = create_paintable_object_v2(true);
    paint_object(car.as_ref());

    let house: Box<dyn Paint> = create_paintable_object_v2(false);
    paint_object(house.as_ref());
}

pub fn trait_objects_vector() {
    let n1 = r#"
    pod: Vector of Implementation
    - `let vector: Vec<&dyn Paint>`
    ---"#;
    println!("{n1}");
    let car = Car {};
    let house = House {};
    let paintable_objects: Vec<&dyn Paint> = vec![&car, &house];
    println!("paintable elements: {}", paintable_objects.len())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_trait_objects() {
        trait_objects()
    }

    #[test]
    fn run_trait_objects_one_concrete_type() {
        trait_objects_one_concrete_type();
    }

    #[test]
    fn run_trait_objects_static_dynamic() {
        trait_objects_static_dynamic();
    }

    #[test]
    fn run_trait_objects_dynamic_types() {
        trait_objects_dynamic_types();
    }

    #[test]
    fn run_trait_objects_vector() {
        trait_objects_vector();
    }
}
