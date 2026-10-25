pub fn draw_line() {
    println!(" > func: draw line without color");
}

#[cfg(feature = "color")]
pub mod color {
    pub use rgb::RGB;

    pub fn draw_line(x: i32, y: i32, color: &RGB<u16>) {
        println!(" > func: draw line with color");
        println!(" > x: {x}, y: {y}, color: {color:?}");
    }
}

#[cfg(feature = "shapes")]
pub mod shapes {
    use rgb::RGB;
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Serialize, Deserialize)]
    pub struct Rectangle {
        pub color: RGB<u16>,
        pub width: u32,
        pub height: u32,
    }
}
