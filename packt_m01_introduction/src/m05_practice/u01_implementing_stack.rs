//! # Implementing Stack

fn stack() {
    let n1 = r#"
    ---
    pod: Stack
    - ADT: Abstract Data Type
    - LIFO: Last In First Out
    - Terminology: `top`, `push` and `pop`

    method: `with_capacity()`
    - Constructs a new, empty Vec<T> with at least the specified capacity.

    method: `pop()`
    - Removes the last element from a vector and returns it, or `None` if it is empty.

    method: `push()`
    - Appends an element to the back of a collection.
    - Panics if the new capacity exceeds isize::MAX bytes.

    method: `unwrap()`
    - Returns the contained `Some` value, consuming the `self` value.
    - Panics if the `self` value equals `None`
    - Instead, prefer to use pattern matching and handle the `None` case explicitly.
    - Or call `unwrap_or`, `unwrap_or_else`, or `unwrap_or_default`.
    - In functions returning `Option`, you can use the `?` (try) operator.
    ---"#;
    println!("{n1}");
}

fn new_stack(capacity: usize) -> Vec<u32> {
    println!(" > new stack capacity: {capacity}");
    Vec::with_capacity(capacity)
}

fn pop(stack: &mut Vec<u32>) -> Option<u32> {
    println!(" > pop value");
    stack.pop()
}

fn push(stack: &mut Vec<u32>, value: u32, capacity: usize) {
    println!(" > push value: {value}");
    if stack.len() == capacity {
        println!(" > error: max capacity reached");
    } else {
        stack.push(value);
        println!(" > success");
    }
}

fn size(stack: &Vec<u32>) -> usize {
    println!(" > size: {}", stack.len());
    stack.len()
}

fn stack_custom() {
    println!("Stack");

    let capacity: usize = 3;
    let mut stack = new_stack(capacity);
    println!(" > status: {stack:?}");

    push(&mut stack, 20, capacity);
    push(&mut stack, 40, capacity);
    push(&mut stack, 60, capacity);
    push(&mut stack, 90, capacity);
    println!(" > status: {stack:?}");

    let popped = pop(&mut stack);
    println!(" > popped: {:?}", popped);
    println!(" > status: {stack:?}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_stack() {
        stack()
    }

    #[test]
    fn run_stack_custom() {
        stack_custom()
    }
}
