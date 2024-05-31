#![allow(unused)]

trait UIComponent {
    fn render(&self) {
        println!("Rendering component...");
    }
}

struct Button {
    text: String
}

impl UIComponent for Button {}

// Error: recuervie type has infinite size, insert some indirection
struct Container {
    name: String, 
    // It now compiles because Box is a simple pointer with a know size
    child: Box<Container>
}

impl UIComponent for Container {}

fn main() {
    
    // The button is stored on the stack
    let button_a1 = Button{ text: "button a1".to_owned() };

    // If we want to store the button on the heap, we can use the Box smart pointer
    // The box smart pointer is already included in the Rust prelude
    // It gives you single ownership of something stored on the heap
    let button_b1 = Box::new(Button{ text: "button b1".to_owned() });

    // Use case: to avoid copying large amounts of data when transferring ownership
    
    // Ownership of the buttons is transferred
    // When transferring ownership data is copied on th stack

    // In this case the entire button is copied
    let button_a2 = button_a1;

    // In this case only the box smart pointer is copied
    // The actual button is still stored on the heap
    let button_b2 = button_b1;


    // Use case: in combination with trait objects
    // We want to store any type that implements the UIComponent trait
    let components: Vec<Box<dyn UIComponent>> = vec![
        Box::new(button_a2),
        button_b2
    ];

    // Use case: when you have a type of an unknown size and you want to use it in
    // a context where the exact size is required, for instance: recursive types.


    println!("Hello, world!");
}  