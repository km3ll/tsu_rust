#![allow(unused)]

fn main() {

    // Like arrays, vectors hold a sequence of elements of the same type.
    // Vectors are growable and allocate memory on the heap.

    // 1. new() function
    let mut vector1: Vec<String> = Vec::new();
    vector1.push(String::from("One"));
    vector1.push(String::from("Two"));

    // When adding elements to a vector elements are moved into the vector, so
    // the vector has ownership of them. This means that when the vector is dropped
    // all of its elements are dropped as well
    vector1.push(String::from("Three"));

    // 2. vec! macro
    let vector2: Vec<i32> = vec![1, 2, 3];

    // Indexing into a Vector
    // 3. Brackets syntax
    // We cannot move a value out of the vector using this syntax because that would
    // leave the vector in an invalid state
    let first_word = &vector1[0]; // can panic

    // A safe way to remove an element is using the remove() method
    // Remove will remove an element at a given index, moving it out of the vector,
    // but it will also shift all the elements after it to the left, such that
    // the vector stays in a valid state.
    let mut vector3: Vec<String> = Vec::new();
    vector3.push(String::from("Uno"));
    vector3.push(String::from("Dos"));
    vector3.push(String::from("Tres"));
    vector3.remove(0);

    // 4. get() method
    let maybe_word: Option<&String> = vector1.get(0); // doesn't panic
    if let Some(word) = maybe_word {
        println!("{word}");
    }

    // Iterating over elements
    // 5. Adding an exclamation mark at the then of each string, by mutably borrowing vector1
    for s in &mut vector1 {
        s.push_str("!")
    }

    // This time we are borrowing vector1 immutably
    for s in &vector1 {
        println!("{s}");
    }

    // Using a for-loop that consumes a vector
    // We are moving the elements from vector1 into vector4, because we are taking vector1 by value
    let mut vector4: Vec<String> = Vec::new();

    //for s in vector1.into_iter() {
    for s in vector1 {
        vector4.push(s);
    }

    // After this for-loop, vector1 is no longer valid
    // vector1.get(1); // Error: value borrowed here after move

}
