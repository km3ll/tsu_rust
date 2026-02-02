//! # Storing Lists of Values with Vectors

#[derive(Debug)]
enum Cell {
	Int(i32),
	Float(f64),
	Text(String),
}

fn vectors_collections() {
	let n1 = r#"
	pod: Collections
	- Vector: a variable number of values next to each other
	- String: a collection of characters
	- HashMap: associates a value with a specific key
	---"#;
	println!("{n1}");
}

fn vectors_definition() {
	let n1 = r#"
	pod: Vectors
	- Store more than one value next to each other in memory
	- Store values of the same type
	- Reads via indexing or by using the 'get' method
	- Indexed by number, starting at zero
	- Indexing method panics when referencing a nonexistent element
	---
	pod: macro: vec!()
	- Creates a vector that holds the values you give it
	---"#;
	println!("{n1}");

	let vec1: Vec<i32> = Vec::new();
	println!("Vectors: vec1: {vec1:?}");

	let vec2: Vec<i32> = vec![1, 2, 3];
	println!("Vectors: vec2: {vec2:?}");
}

fn vectors_update() {
	let mut vec1 = Vec::new();
	vec1.push(5);
	vec1.push(6);
	vec1.push(7);

	println!("Vectors: vec1: {vec1:?}");
}

fn vectors_read() {
	let n1 = r#"
	pod: Vector Ownership and Borrowing Rules
	- Vectors put the values next to each other in memory
	- Adding elements might require allocating new memory and copying the old elements into new space
	- In that case, references to elements would be pointing to deallocated memory
	- Dropping a vector drops its elements
	---"#;
	println!("{n1}");

	let vector = vec![1, 2, 3, 4, 5];

	let e3: &i32 = &vector[2];
	println!("Vectors: vector: {vector:?}, 3rd element: {e3}");

	let result: Option<&i32> = vector.get(2);
	match result {
		Some(e2) => println!("Vectors: vector: {vector:?}, 2nd element: {e2}"),
		None => println!("Vectors: there is no 2nd element"),
	}
}

fn vectors_iterating() {
	let vec1 = vec![20, 40, 60];
	println!("Vectors: vec1: {vec1:?}");

	for elem in &vec1 {
		println!("> {elem}");
	}

	let mut vec2 = vec![10, 20, 25];
	println!("Vectors: mut vec2: {vec2:?}");
	for i in &mut vec2 {
		*i += 50;
	}
	println!("Vectors: mut vec2: {vec2:?}");
}

fn vectors_enums() {
	let vec1 = vec![
		Cell::Int(3),
		Cell::Float(4.5),
		Cell::Text(String::from("blue")),
	];
	println!("Vectors: enums vec1: {vec1:?}");
}

fn vectors_dropping() {
	let mut vec1 = vec![1, 2, 3, 4];
	println!("Vectors: vec1.pop: {vec1:?}");
	let e3 = vec1.pop();
	println!("Vectors: vec1.pop: {vec1:?}");
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn run_vectors_collections() {
		vectors_collections();
	}

	#[test]
	fn run_vectors_definition() {
		vectors_definition();
	}

	#[test]
	fn run_vectors_update() {
		vectors_update();
	}

	#[test]
	fn run_vectors_read() {
		vectors_read();
	}

	#[test]
	fn run_vectors_iterating() {
		vectors_iterating();
	}

	#[test]
	fn run_vectors_enums() {
		vectors_enums();
	}

	#[test]
	fn run_vectors_dropping() {
		vectors_dropping();
	}
}
