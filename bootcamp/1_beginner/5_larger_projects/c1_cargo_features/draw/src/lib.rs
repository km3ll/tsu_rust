/**
 * This library is very small. However, we don't want consumers
 * of the library to pay for behavior that they don't use.
 * 
 * Features allow two things:
 *  - defining parts of code that are conditionally 
 *    compiled, only if a certain feature is turned on.
 *  - defining optional dependencies.
 * 
 * Features reduce compile times and file sizes
 */

pub fn draw_line(x: i32, y: i32) {
    println!("Draw line without color");
}

// Conditionally include code at compile time
// The color module is included only if the color feature is enabled
#[cfg(feature = "color")]
pub mod color {
    pub use rgb::RGB;

    pub fn draw_line(x: i32, y: i32, color: &RGB<i32>) {
        println!("{color}");
        println!("Draw line with color");
    }
}

// This code is not compiled by default
#[cfg(feature = "shapes")]
pub mod shapes {
    use serde::{Serialize, Deserialize};
    use rgb::RGB;

    #[derive(Debug, Serialize, Deserialize)]
    pub struct Rectangle {
        pub color: RGB<i32>,
        pub width: u32,
        pub height: u32,
    }
}