//! # String Reversal Using Stacks

fn new_stack(capacity: usize) -> Vec<char> {
    Vec::with_capacity(capacity)
}

fn pop(stack: &mut Vec<char>) -> Option<char> {
    stack.pop()
}

fn push(stack: &mut Vec<char>, value: char, capacity: usize) {
    if stack.len() == capacity {
        println!(" > error: max capacity reached");
    } else {
        stack.push(value)
    }
}

fn size(stack: &Vec<char>) -> usize {
    stack.len()
}

fn reversal_custom() {
    println!("String Reversal");

    let input = String::from("Hello, Ferris!");
    println!(" > input : {input}");

    let capacity = input.len();
    let mut stack = new_stack(capacity);

    for char in input.chars() {
        push(&mut stack, char, capacity)
    }
    println!(" > stack : {:?}", &stack);

    let mut output = String::new();
    for i in 0..size(&stack) {
        let popped = pop(&mut stack).unwrap();
        output.push(popped)
    }
    println!(" > output: {output}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_reversal_custom() {
        reversal_custom()
    }
}
