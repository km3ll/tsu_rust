//! # Function Types

fn max(a: i32, b: i32) -> i32 {
    println!(" > max: a: {a}, b: {b}");
    if a >= b { a } else { b }
}

fn min(a: i32, b: i32) -> i32 {
    println!(" > min: a: {a}, b: {b}");
    if a <= b { a } else { b }
}

fn prints(value: &str) {
    println!(" > prints: value: {value}");
}

fn orchestrator(value: &str, function: fn(&str) -> ()) {
    println!(" > orchestrator: function {function:?}, value: {value}");
    function(value)
}

fn function_pointer() {
    let n1 = r#"
    ---
    pod: Function Pointer Type
    - Refer to a function whose identity is not necessarily known at compile time
    - Points to executable code within memory
    ---"#;
    println!("{n1}");

    println!("Function Pointer");

    println!(" > mut function: fn(i32, i32) -> i32 = max");
    let mut function: fn(i32, i32) -> i32 = max;

    let res1 = function(15, 30);
    println!(" > function: res1: {res1}");

    function = min;
    println!(" > mut function = min");

    let res2 = function(28, 64);
    println!(" > function: res2: {res2}");
}

fn function_as_parameter() {
    println!("Function Pointers");

    println!(" > outer: function as parameter");
    let printer = prints;
    let result = orchestrator("Ferris!", printer);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_function_pointer() {
        function_pointer()
    }

    #[test]
    fn run_function_as_parameter() {
        function_as_parameter()
    }
}
