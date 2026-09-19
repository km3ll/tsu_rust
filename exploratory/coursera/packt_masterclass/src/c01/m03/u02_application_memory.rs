//! # Application Memory - Heap and Stack

use std::fmt::format;

fn memory() {
    let n1 = r#"
    pod: Memory Segments
    - Code/Text: program instructions and code
    - Static/Global: variables available during whole program execution
    - Stack: function calls, local variables and primitive type variables
    ---
    pod: Memory Allocation
    - The allocation of stack frames and local variables happen at runtime
    - If the stack grows beyond the reserved memory then a Stack Overflow happens
    - Data stored on the stack must have a known fixed size
    - Data stored on the heap does not have a fixed size, is unknown at compile time
    - The heap is less organized than the stack. Objects are not added in sequential order
    - The program requests a certain amount of memory (heap) to the OS
    ---"#;
    println!("{n1}");
}

const MAX_VALUE: i32 = 40_000;

/// # Memory Overview
///
/// ## Heap
/// - "starts" # msg
/// - "starts" # msg_cloned
/// ## Stack
/// square()
/// - num
/// ----------
/// square_sum()
/// - num1 & num2
/// ----------
/// memory_main()
/// - x & y
/// - msg
/// - msg_ref
/// - msg_cloned
/// ----------
/// ## Global
/// `MAX_VALUE`
/// ----------
fn memory_main() {
    let (x, y) = (2, 4);

    let msg: String = String::from("Memory: starts");
    let msg_ref: &String = &msg;
    let msg_cloned: String = msg.clone();

    println!("{msg}");

    let sum_value = square_sum(x, y);
    println!("Memory: sum_value: {sum_value}");
}

fn square_sum(num1: i32, num2: i32) -> i32 {
    let result = square(num1 + num2);
    result
}

fn square(num: i32) -> i32 {
    num * num
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_memory() {
        memory()
    }

    #[test]
    fn run_memory_main() {
        memory_main()
    }
}
