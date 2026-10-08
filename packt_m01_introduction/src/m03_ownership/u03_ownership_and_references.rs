//! # Ownership and References in Functions

fn stack_function(mut stack_count: i32) {
    stack_count = 5_000;
    println!(" > inside stack function i32: {stack_count}");
}

fn heap_function_v1(numbers: Vec<i32>) {
    println!(" > inside function_v1 {:?}", numbers)
}

fn heap_function_v2(numbers: &mut Vec<i32>) {
    numbers.push(10);
    println!(" > inside function_v2 {:?}", numbers)
}

fn stack_value() {
    let n1 = r#"
    ---
    pod: Values in the Stack
    - When we pass a primitive value to a function, it is copied into the new stack frame
    - The move operation does not take place with primitive types
    - Updates of the value inside the function do not affect the value original value
    - Inside the function the parameter is mutable, while outside it can be immutable, because they are two different variables

    pod: Values on the Heap
    - When we pass a variable that is store on the heap to a function, its value gets moved
    - The owner of the value changes
    ---"#;
    println!("{n1}");

    println!("Stack");
    let stack_count: i32 = 10;
    println!(" > outside stack function i32: {stack_count}");
    stack_function(stack_count);
    println!(" > outside stack function i32: {stack_count}");
}

fn heap_value() {
    println!("Heap");
    let numbers_v1 = vec![1, 2, 3];
    println!(" > outside heap function_v1 Vec<i32>: {:?}", numbers_v1);
    heap_function_v1(numbers_v1); // vector moved here
    // println!("Heap: after function numbers_v1: {:#?}", numbers_v1);

    let mut numbers_v2 = vec![4, 5, 6];
    println!(" > outside heap function_2 Vec<i32>: {:?}", numbers_v2);
    heap_function_v2(&mut numbers_v2); // vector moved here
    println!(" > outside heap function_2 Vec<i32>: {:?}", numbers_v2);
}

fn heap_merge() {
    let n1 = r#"
    ---
    pod: Conceptual Merge
    - Using references to avoid overhead on the heap when merging variables
    ---"#;
    println!("{n1}");

    println!("Conceptual merge");
    let s1 = String::from("Data1");
    let s2 = String::from("Data2");
    println!(" > s1: {s1}, s2: {s2}");
    let vec1: Vec<&String> = vec![&s1, &s2];
    println!(" > Vec<&String> vec1: {vec1:?}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_stack_value() {
        stack_value()
    }

    #[test]
    fn run_heap_value() {
        heap_value()
    }

    #[test]
    fn run_heap_merge() {
        heap_merge()
    }
}
