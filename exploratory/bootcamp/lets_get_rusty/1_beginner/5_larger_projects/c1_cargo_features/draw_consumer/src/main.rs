// Bring color module into scope
use draw::color;
use draw::color::RGB;
use draw::shapes::Rectangle;

fn main() {

    // We only have access to the top level color module and draw_line function
    // The draw_line function is not gated behind any features, and the color
    // module is gated behind the color feature --enabled by default
    draw::draw_line(32, 32 );

    let color: RGB<i32> = RGB {
        r: 247,
        g: 76,
        b: 0,
    };
    color::draw_line(32, 32, &color);
    
    let rectangle = Rectangle {
        color,
        width: 32,
        height: 32,
    };
    println!("{rectangle:?}");

}
