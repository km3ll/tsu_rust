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

/**
 * pod: Instance stored on the stack
 */
fn box_smart_pointer_on_stack() {
	let b1 = Button {
		text: "Send".to_owned(),
	};
	println!("b1: {:?}", b1);
}

/**
 * pod: Box::new()
 * - Instance stored on the heap
 * - Gives single ownership of something stored on the heap
 */
fn box_smart_pointer_on_heap() {
	let b2: Box<Button> = Box::new(Button {
		text: "Send".to_owned(),
	});
	println!("b2: {:?}", b2);
}

/**
 * pod: Use of Box smart pointer
 * - To avoid copying large amounts of data when transferring ownership
 */
fn box_smart_pointer_avoid_copy() {
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

/**
 * pod: Use of Box smart pointer
 * - In combination with trait objects
 */
fn box_smart_pointer_trait_objects() {
	let b1: Box<Button> = Box::new(Button {
		text: "Send".to_owned(),
	});
	let b2: Box<Button> = Box::new(Button {
		text: "Cancel".to_owned(),
	});
	let components: Vec<Box<dyn UIComponent>> = vec![b1, b2];
}

/**
 * pod: Use of Box smart pointer
 * - You have a type of unknown size and you want to use it
 *   in a context where the exact size is required.
 *   For instance: recursive types
 * - We cannot know the size of Container at compile time
 * - The size of 'child' is the size of Box
 */
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
