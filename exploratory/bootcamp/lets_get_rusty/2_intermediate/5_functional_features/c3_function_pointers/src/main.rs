fn main() {
    
    println!("Hello, world!");

    // *x dereferences the reference, so we have the actual value
    let greater_than_closure = |x: &i32| *x > 10;
    let less_than_closure = |x: &i32| *x < 20;

    let result_1 = both_are_true_v1(greater_than_closure, less_than_closure, &15);
    let result_2 = both_are_true_v1(greater_than_closure, less_than_closure, &9);
    
    // Closures can be coerced to function pointers if they do not capture any variables.
    // To get around mismatch errors use closures as arguments so you can pass in
    // both closures and function pointers.

    let result_3 = both_are_true_v1(greater_than_function, less_than_function, &15);
    let result_4 = both_are_true_v1(greater_than_function, less_than_function, &9);

    let result_5 = both_are_true_v2(greater_than_function, less_than_function, &15);
    let result_6 = both_are_true_v2(greater_than_function, less_than_function, &9);

    println!("result 1: {}", result_1);
    println!("result 2: {}", result_2);
    println!("result 3: {}", result_3);
    println!("result 4: {}", result_4);
    println!("result 5: {}", result_5);
    println!("result 6: {}", result_6);

}

// Function pointer are similar to closures except they don't capture
// variables in their environment
fn less_than_function(x: &i32) -> bool {
    *x < 20
}

fn greater_than_function(x: &i32) -> bool {
    *x > 10
}

// You can think of this function as taking in a reference to some value
// and two validators. This function accepts closures (and function pointers)
fn both_are_true_v1<T, U, V>(f1: T, f2: U, item: &V) -> bool 
    where T: Fn(&V) -> bool, U: Fn(&V) -> bool {
    f1(item) && f2(item)
}

// This function accepts function pointers
// Function pointers are a concrete type represented with lowercase 'fn'
// Becaute they are concrete types we don't have to use generics
fn both_are_true_v2<V>(f1: fn(&V) -> bool, f2: fn(&V) -> bool, item: &V) -> bool {
    f1(item) && f2(item)
}