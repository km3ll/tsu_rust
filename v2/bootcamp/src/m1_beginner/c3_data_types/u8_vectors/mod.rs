pub fn vectors() {
    println!("----------");
    println!("Vectors");

    /**
     * Vectors
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
    v1.push(String::from("Four"));
    v1.push(String::from("Five"));
    println!(" > v1: {:#?}", v1);

    /**
     * Macro: vec!
     * - Define a vector and initialize it
     */
    let v2: Vec<i32> = vec![1, 2, 3, 4, 5];
    println!(" > v2: {:#?}", v2);

    /**
     * Index using brackets syntax
     * - Can panic
     * - Cannot move elements using this syntax, we can borrow instead
     */
    let s1: &String = &v1[0]; // can panic
    println!(" > s1: {s1}");

    /***
     * Index using the 'get' method
     * - Does not panic, because it returns an option instead
     */
    let s2 = v2.get(10);
    println!(" > s2: {:#?}", s2);

    /***
     * Removing elements with 'remove' method
     * - Removes at a given index
     * - Shifts all the elements after it to the left
     */
    v1.remove(3);
    println!(" > v1: {:#?}", v1);

    /**
     * Iterating over elements within a vector
     */
    for s in &mut v1 {
        s.push_str("!");
    }
    println!(" > v1: {:#?}", v1);

    for s in &v1 {
        println!(" > s: {s}")
    }

    /**
     * Iterating with a for-loop that consumes a vector
     * - Taking the first vector by value
     * - After the for-loop call v2 is no longer valid
     */
    let mut v3: Vec<i32> = Vec::new();

    // for n in v2.into_iter() {
    for n in v2 { // v2 by value
        v3.push(n);
    }
    println!(" > v3: {:#?}", v3);
    //let i = v2.get(0); Error: borrow of moved value
} // v1 is dropped