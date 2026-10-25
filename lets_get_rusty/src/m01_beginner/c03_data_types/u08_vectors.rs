//! # Vectors

pub fn vectors_definition() {
    let n1 = r#"
    ---
	pod: Vectors
	- Hold a sequence of elements of the same type
	- Growable and allocate memory on the heap
	- When adding elements, they are moved into the vector
	- The vector has ownership of the elements
	- When the vector is dropped, all of its elements are also dropped
	---"#;
    println!("{n1}");

    println!("Vectors");
    let mut v1: Vec<String> = Vec::new();
    println!(" > mut v1: {v1:?}");

    v1.push(String::from("One"));
    v1.push(String::from("Two"));
    v1.push(String::from("Three"));
    println!(" > mut v1: {v1:?}");
}

pub fn vectors_macro() {
    let n1 = r#"
    ---
	macro: `vec!()`
	- Defines a vector and initializes it
	---"#;
    println!("{n1}");

    println!("Vectors");
    let v2: Vec<i32> = vec![1, 2, 3];
    println!(" > v2: {v2:?}");
}

pub fn vectors_index_brackets() {
    let n1 = r#"
    ---
	pod: Vector: Brackets Syntax
	- Can panic when passed an invalid index
	- Cannot move elements using this syntax because it would leave the vector in invalid state.
	- We can borrow instead
	---"#;
    println!("{n1}");

    println!("Vectors");
    let v1: Vec<i32> = vec![10, 20, 30];
    println!("> v1: {v1:?}");

    let s1: &i32 = &v1[0];
    println!("> &v1[0]: {s1}");
}

pub fn vectors_index_get() {
    let n1 = r#"
    ---
	pod: Vector: `get()` Method
	- Does not panic, because it returns an option instead
	---"#;
    println!("{n1}");

    println!("Vectors");
    let v1: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7, 8];
    println!(" > v1: {v1:?}");

    let s1: Option<&i32> = v1.get(3);
    println!(" > v1.get(3): {s1:?}");

    println!(" > if-let:");
    if let Some(value) = v1.get(4) {
        println!(" > v1.get(4): value: {value}")
    }
}

pub fn vectors_remove() {
    let n1 = r#"
    ---
	pod: Vector: `remove()` Method
	- Removes an element at a given index
	- Shifts all the elements after it to the left
	---"#;
    println!("{n1}");

    println!("Vectors");
    let mut v1 = vec![1, 2, 3, 4, 5];
    println!(" > mut v1: {v1:?}");

    v1.remove(2);
    println!(" > remove(2)");
    println!(" > mut v1: {v1:?}");
}

pub fn vectors_iteration_mutable() {
    println!("Vectors");

    let mut v1: Vec<String> = Vec::new();
    v1.push(String::from("One"));
    v1.push(String::from("Two"));
    v1.push(String::from("Three"));
    println!(" > mut v1: {v1:?}");

    println!(" > &mut push_str()");
    for value in &mut v1 {
        value.push_str("!");
    }
    println!(" > mut v1: {v1:?}");
}

pub fn vectors_iteration_immutable() {
    println!("Vectors");
    let v1: Vec<String> = vec![
        String::from("One"),
        String::from("Two"),
        String::from("Three"),
    ];
    println!(" > v1: {v1:?}");

    println!(" > for-loop:");
    for value in &v1 {
        println!(" > {value}")
    }
}

pub fn vectors_iteration_consuming() {
    let n1 = r#"
    ---
	pod: Vector For-Loop
	- Taking the first vector by value
	- After the for-loop call v1 is no longer valid
	---"#;
    println!("{n1}");

    println!("Vectors");
    let mut v1: Vec<String> = vec![
        String::from("One"),
        String::from("Two"),
        String::from("Three"),
    ];
    println!(" > mut v1: {v1:?}");

    let mut v2: Vec<String> = Vec::new();
    println!(" > mut v2: {v2:?}");

    println!(" > for-loop v1:");
    for value in v1 {
        println!(" > push({value})");
        v2.push(value);
    }

    // println!("v1 {:#?}", v1); | Error: borrow of moved value
    // let i =  v1.get(0); | Error: borrow of moved value
    println!(" > v1: Error: borrow of moved value");
    println!(" > v2 {v2:?}");
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
