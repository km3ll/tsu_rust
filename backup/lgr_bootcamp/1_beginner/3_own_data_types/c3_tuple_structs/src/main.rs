#!(allow[unused])

fn main() {

    // Tuples don't define new types
    let rgb_color: (i32, i32, i32, ) = (255, 106, 0);
    let cmyk_color: (i32, i32, i32, i32) = (0, 58, 100, 0);

    // tuple structs
    struct RGB(i32, i32, i32, );
    struct CMYK(i32, i32, i32, i32);

    let color1: RGB = RGB(255, 106, 0);
    let color2: CMYK = CMYK(0, 58, 100, 0);

    // unit-like structs
    struct MyStruct;
    println!("First element of rgb_color is: {:?}", rgb_color);

}