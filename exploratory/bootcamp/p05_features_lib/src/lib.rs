#![allow(unused_variables)]

pub fn draw_line(x: i32, y: i32) {
    // draw line without color
}

// pod: color module enabled only if 'color' feature is enabled
#[cfg(feature = "color")]
pub mod color {
    pub use rgb::RGB;
    pub fn draw_line(x: i32, y: i32, color: &RGB<u16>) {
        println!("color: {color}")
        // draw line with color
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
