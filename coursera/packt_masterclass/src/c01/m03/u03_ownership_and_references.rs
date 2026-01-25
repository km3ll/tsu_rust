//! # Ownership and References in Functions

fn stack_function(mut stack_count: i32) {
    stack_count = 5_000;
    println!(" > inside function count: {stack_count}");
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
    pod: Values in the Stack
    - When we pass a primitive value to a function, it is copied into the new stack frame
    - The move operation does not take place with primitive types
    - Updates of the value inside the function do not affect the value original value
    - Inside the function the parameter is mutable, while outside it can be immutable, because they are two different variables
    ---
    pod: Values on the Heap
    - When we pass a variable that is store on the heap to a function, its value get moved
    - The owner of the value changes
    ---"#;
    println!("{n1}");

    let stack_count: i32 = 10;
    println!("Stack: before function count: {stack_count}");
    stack_function(stack_count);
    println!("Stack: after function count: {stack_count}");
}

fn heap_value() {
    let numbers_v1 = vec![1, 2, 3];
    println!("Heap: before function numbers_v1: {:?}", numbers_v1);
    heap_function_v1(numbers_v1); // vector moved here
    // println!("Heap: after function numbers_v1: {:#?}", numbers_v1);

    let mut numbers_v2 = vec![4, 5, 6];
    println!("Heap: before function numbers_v2: {:?}", numbers_v2);
    heap_function_v2(&mut numbers_v2); // vector moved here
    println!("Heap: after function numbers_v2: {:?}", numbers_v2);
}

fn heap_merge() {
    let n1 = r#"
    pod: Conceptual Merge
    - Using references to avoid overhead on the heap when merging variables
    ---"#;
    println!("{n1}");

    let data1 = String::from("Data1");
    let data2 = String::from("Data2");
    let data: Vec<&String> = vec![&data1, &data2];
    println!("Conceptual merge: {:?}", data);
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
