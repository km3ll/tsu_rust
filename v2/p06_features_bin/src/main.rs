use p05_features_lib::color;

fn main() {
    p05_features_lib::draw_line(32, 32);
    let color = color::RGB {
        r: 247,
        g: 76,
        b: 0,
    };
    color::draw_line(32, 32, &color);

    let square = p05_features_lib::shapes::Rectangle {
        color,
        width: 32,
        height: 32,
    };
    println!("square: {:?}", square);
}
