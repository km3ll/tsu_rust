//! # Box Smart Pointer

trait UIComponent {
    fn render(&self) {
        println!("Rendering...")
    }
}

#[derive(Debug)]
struct Button {
    text: String,
}

impl UIComponent for Button {}

fn box_smart_pointer_on_stack() {
    // Instance stored on the slack
    let b1 = Button {
        text: "Send".to_owned(),
    };
    println!("b1: {:?}", b1);
}

fn box_smart_pointer_on_heap() {
    let n1 = r#"
    pod: Box::new()
    - Instance stored on the heap
    - Gives single ownership of something stored on the heap
    ---"#;
    println!("{n1}");

    let b2: Box<Button> = Box::new(Button {
        text: "Send".to_owned(),
    });
    println!("b2: {:?}", b2);
}

fn box_smart_pointer_avoid_copy() {
    let n1 = r#"
    pod: Box Smart Pointer Usage
    - To avoid copying large amounts of data when transferring ownership
    - In combination with trait objects
    - You have a type of unknown size and you want to use it in a context where the exact size is required
    - For instance: recursive types: we cannot know the size of Container at compile time
    ---"#;
    println!("{n1}");

    let b1: Button = Button {
        text: "Send".to_owned(),
    };
    let b2: Box<Button> = Box::new(Button {
        text: "Send".to_owned(),
    });

    // Ownership is transferred here

    // The entire button 1 is copied around the stack
    let b3: Button = b1;

    // Only the Box smart pointer will be copied
    let b4: Box<Button> = b2;

    println!("b3: {:?}", b3);
    println!("b4: {:?}", b4);
}

fn box_smart_pointer_trait_objects() {
    let b1: Box<Button> = Box::new(Button {
        text: "Send".to_owned(),
    });
    let b2: Box<Button> = Box::new(Button {
        text: "Cancel".to_owned(),
    });
    let components: Vec<Box<dyn UIComponent>> = vec![b1, b2];
}

#[derive(Debug)]
struct Container {
    name: String,
    child: Box<Container>, // error: recursive type `Container` has infinite size
                           // child: Container
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_box_smart_pointer_on_stack() {
        box_smart_pointer_on_stack();
    }

    #[test]
    fn run_box_smart_pointer_on_heap() {
        box_smart_pointer_on_heap();
    }

    #[test]
    fn run_box_smart_pointer_avoid_copy() {
        box_smart_pointer_avoid_copy();
    }

    #[test]
    fn run_box_smart_pointer_trait_objects() {
        box_smart_pointer_trait_objects();
    }
}
