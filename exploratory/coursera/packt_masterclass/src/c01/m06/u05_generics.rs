//! # Generics

use std::fmt::Display;
use std::ops::Mul;

fn square_def_1<T: Mul<Output = T> + Copy>(value: T) -> T {
    value * value
}

fn square_def_2<T>(value: T) -> T
where
    T: Mul<Output = T> + Copy,
{
    value * value
}

#[derive(Debug)]
struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U>
where
    T: Display,
    U: Display,
{
    fn print(&self) {
        println!(" > (x: {}, y: {})", self.x, self.y);
    }
}

fn generics() {
    let n1 = r#"
    pod: Generics
    - Programs are written in terms of to-be-specified-later types
    ---
    pod: Traits with Generics
    - Add, Copy, Mul<Output = T>
    ---"#;
    println!("{n1}");

    let sq1: i32 = square_def_1(5);
    let sq2: f64 = square_def_2(5.9);
    println!("Generics");
    println!(" > square i32: {sq1}");
    println!(" > square f64: {sq2}");
}

fn generics_in_structs() {
    println!("Generics");

    let p1: Point<i32, i32> = Point { x: 10, y: 20 };
    let p2: Point<f64, f64> = Point { x: 0.25, y: 11.3 };
    let p3: Point<i32, f64> = Point { x: 4, y: 8.2 };

    println!(" p1: {p1:?}");
    println!(" p2: {p2:?}");
    println!(" p3: {p3:?}");

    let p4: Point<i32, i32> = Point { x: 25, y: 50 };
    p4.print()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_generics() {
        generics()
    }

    #[test]
    fn run_generics_in_structs() {
        generics_in_structs()
    }
}
