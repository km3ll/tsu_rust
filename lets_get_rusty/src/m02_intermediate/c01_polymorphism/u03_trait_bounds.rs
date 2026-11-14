//! # Trait Bounds

#[derive(Debug)]
struct Car {
    year: u16,
}

trait Paint {
    fn paint(&self, color: String) {
        println!(" > meth<Paint>: paint({color})")
    }
}

trait Park {
    fn park(&self) {
        println!(" > meth<Park>: park()")
    }
}

impl Paint for Car {}
impl Park for Car {}

fn paint_red_v1<T: Paint>(object: &T) {
    println!(" > func: paint_red(): T: Paint");
    object.paint("red".to_owned());
}

fn paint_red_v2(object: &impl Paint) {
    println!(" > func: paint_red(): &impl Paint");
    object.paint("red".to_owned());
}

fn paint_red_v3<T>(object: &T)
where
    T: Paint + Park,
{
    println!(" > impl: paint_red(): where T: Paint + Park");
    object.paint("red".to_owned());
}

fn trait_bounds_colon_syntax() {
    println!("Trait Bounds");
    let car = Car { year: 2020 };
    println!("> main: car: {car:?}");
    paint_red_v1(&car);
}

pub fn trait_bounds() {
    let n1 = r#"
    ---
    pod:  Trait Bound Syntax
    - 1. T: Paint
    - 2. impl syntax
    - 3. where syntax (multiple)

    pod: Trait Bound
    - Can be used as return types
    ---"#;
    println!("{n1}");
}

pub fn trait_bounds_impl_syntax() {
    println!("Trait Bounds");
    let car = Car { year: 2022 };
    println!(" > main: car: {car:?}");
    paint_red_v2(&car);
}

fn trait_bounds_where_syntax() {
    println!("Trait Bounds");
    let car = Car { year: 2023 };
    println!(" > main: car: {car:?}");
    paint_red_v3(&car);
}

fn create_paintable_object() -> impl Paint {
    let car = Car { year: 2021 };
    println!(" > func: return paintable: {car:?}");
    return car;
}

fn trait_bounds_return_type() {
    println!("Trait Bounds");
    println!(" > main");
    let p1 = create_paintable_object();
    println!(" > main: p1");
    p1.paint("blue".to_owned());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_trait_bounds() {
        trait_bounds()
    }

    #[test]
    fn run_trait_bounds_colon_syntax() {
        trait_bounds_impl_syntax();
    }

    #[test]
    fn trait_bounds_colon_syntax() {
        trait_bounds_impl_syntax();
    }

    #[test]
    fn run_trait_bounds_where_syntax() {
        trait_bounds_where_syntax();
    }

    #[test]
    fn run_trait_bounds_return_type() {
        trait_bounds_return_type();
    }
}
