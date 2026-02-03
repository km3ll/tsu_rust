//! # Implementing Stack

fn stack() {
    let n1 = r#"
    pod: Stack
    - ADT - Abstract Data Type
    - LIFO - Last In First Out
    - Terminology: `top`, `push` and `pop`
    ---
    pod: Method Vec::with_capacity()
    - Constructs a new, empty Vec<T> with at least the specified capacity.
    ---"#;
    println!("{n1}");
}

fn new_stack(capacity: usize) -> Vec<u32> {
    Vec::with_capacity(capacity)
}

fn pop(stack: &mut Vec<u32>) -> Option<u32> {
    stack.pop()
}

fn push(stack: &mut Vec<u32>, value: u32, capacity: usize) {
    if stack.len() == capacity {
        println!("Stack: max capacity reached: {capacity}");
    } else {
        stack.push(value)
    }
}

fn size(stack: &Vec<u32>) -> usize {
    println!("Stack: size: {}", stack.len());
    stack.len()
}

fn stack_custom() {
    println!("Stack");

    let capacity: usize = 3;
    let mut stack = new_stack(capacity);
    println!(" > new: {:?}", stack);

    push(&mut stack, 20, capacity);
    push(&mut stack, 40, capacity);
    push(&mut stack, 60, capacity);
    println!(" > push: {:?}", stack);

    let popped = pop(&mut stack);
    println!(" > pop: {:?}", stack);
    println!(" > popped: {:?}", popped);
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
