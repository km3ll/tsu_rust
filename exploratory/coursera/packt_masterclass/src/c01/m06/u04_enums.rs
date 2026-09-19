//! # Enums

#[derive(Debug)]
enum Conveyance {
    Air = 30,
    Car = 15,
    Train = 20,
}

impl Conveyance {
    fn travel_allowance(&self, miles: f32) -> f32 {
        match self {
            Conveyance::Air => miles * 14.3,
            Conveyance::Car => miles * 18.6,
            Conveyance::Train => miles * 30.7,
        }
    }
}

#[derive(Debug)]
enum ConveyanceV2 {
    Air(f32),
    Car(f32),
    Train(i32),
    Walk,
}

impl ConveyanceV2 {
    fn travel_allowance(&self) -> f32 {
        match self {
            ConveyanceV2::Air(miles) => miles * 14.3,
            ConveyanceV2::Car(miles) => miles * 18.6,
            ConveyanceV2::Train(miles) => *miles as f32 * 30.7,
            ConveyanceV2::Walk => 0.0,
        }
    }
}

#[derive(Debug)]
enum Color {
    Blue,
    Green,
    Red,
}

fn enumerators() {
    let n1 = r#"
    pod: Enumerator
    - Data type consisted of named value elements or variants
    - Can have implementation blocks
    - Can be used in vectors (single type)
    ---
    pod: Enumerator Variant
    - Its numeric values start at Zero `Car as i32`
    - Can be defined with associated values
    ---"#;
    println!("{n1}");

    let c1: Conveyance = Conveyance::Car;
    println!("Enums");
    println!(" > c1: {c1:?}");
    println!(" > c1 as i32: {}", c1 as i32);

    let c2: Color = Color::Red;
    println!(" > c2: {c2:?}");
    println!(" > c2 as i32: {}", c2 as i32);
}

fn enumerators_conveyance() {
    let miles: f32 = 60.0;
    let conveyance = Conveyance::Train;
    let allowance = conveyance.travel_allowance(miles);
    println!(
        "Enums: conveyance: {:?}, allowance: {:?}",
        conveyance, allowance
    );
}

fn enumerators_data() {
    let conveyance = ConveyanceV2::Air(23.7);
    let allowance = conveyance.travel_allowance();
    println!(
        "Enums: conveyance_v2: {:?}, allowance_v2: {:?}",
        conveyance, allowance
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_enumerators() {
        enumerators()
    }

    #[test]
    fn run_enumerators_conveyance() {
        enumerators_conveyance()
    }

    #[test]
    fn run_enumerators_data() {
        enumerators_data()
    }
}
