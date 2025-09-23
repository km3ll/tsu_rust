pub fn vectors_definition() {
	/**
	 * pod: Vectors
	 * - Hold a sequence of elements of the same type
	 * - Growable and allocate memory on the heap
	 * - When adding elements, they are moved into the vector
	 * - The vector has ownership of the elements
	 * - When the vector is dropped, all of its elements are also dropped
	 */
	let mut v1: Vec<String> = Vec::new();
	v1.push(String::from("One"));
	v1.push(String::from("Two"));
	v1.push(String::from("Three"));
	println!("v1: {:#?}", v1);
}

pub fn vectors_macro() {
	/**
	 * pod: vec! macro defines a vector and initialize it
	 */
	let v2: Vec<i32> = vec![1, 2, 3];
	println!("v2: {:#?}", v2);
}

pub fn vectors_index_brackets() {
	/**
	 * pod: Indexing vector using brackets syntax
	 * - Can panic when passed an invalid index
	 * - Cannot move elements using this syntax because
	 *   it would leave the vector in invalid state.
	 *   We can borrow instead
	 */
	let v1: Vec<i32> = vec![10, 20, 30];
	let s1: &i32 = &v1[0];
	println!("s1: {s1}");
}

pub fn vectors_index_get() {
	/***
	 * Index using the 'get' method
	 * - Does not panic, because it returns an option instead
	 */
	let v1: Vec<i32> = vec![1, 2, 3, 4, 5];
	let s1: Option<&i32> = v1.get(3);
	println!("s1: {:#?}", s1);

	let index: usize = 3;
	if let Some(value) = v1.get(index) {
		println!("index: {index}, value: {value}")
	}
}

pub fn vectors_remove() {
	/***
	 * pod: Removing elements with 'remove' method
	 * - Removes an element at a given index
	 * - Shifts all the elements after it to the left
	 */
	let mut v1 = vec![1, 2, 3, 4, 5];
	v1.remove(2);
	println!("v1: {:#?}", v1);
}

pub fn vectors_iteration_mutable() {
	let mut v1: Vec<String> = Vec::new();
	v1.push(String::from("One"));
	v1.push(String::from("Two"));
	v1.push(String::from("Three"));

	for value in &mut v1 {
		value.push_str("!");
	}
	println!("v1: {:#?}", v1);
}

pub fn vectors_iteration_immutable() {
	let v1: Vec<String> = vec![
		String::from("One"),
		String::from("Two"),
		String::from("Three"),
	];

	for value in &v1 {
		println!("{value}")
	}
}

pub fn vectors_iteration_consuming() {
	let mut v1: Vec<String> = vec![
		String::from("One"),
		String::from("Two"),
		String::from("Three"),
	];

	let mut v2: Vec<String> = Vec::new();

	/**
	 * pod: Iterating with a for-loop that consumes a vector
	 * - Taking the first vector by value
	 * - After the for-loop call v2 is no longer valid
	 */
	for value in v1 {
		v2.push(value);
	}

	// println!("v1 {:#?}", v1); | Error: borrow of moved value
	// let i =  v1.get(0); | Error: borrow of moved value
	println!("v2 {:#?}", v2);
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn run_vectors_definition() {
		vectors_definition();
	}

	#[test]
	fn run_vectors_macro() {
		vectors_macro();
	}

	#[test]
	fn run_vectors_index_brackets() {
		vectors_index_brackets();
	}

	#[test]
	fn run_vectors_index_get() {
		vectors_index_get();
	}

	#[test]
	fn run_vectors_remove() {
		vectors_remove();
	}

	#[test]
	fn run_vectors_iterating_mutable() {
		vectors_iteration_mutable();
	}

	#[test]
	fn run_vectors_iteration_immutable() {
		vectors_iteration_immutable();
	}

	#[test]
	fn run_vectors_iteration_consuming() {
		vectors_iteration_consuming();
	}
}
