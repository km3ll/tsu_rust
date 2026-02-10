//! # Function Types

fn max(a: i32, b: i32) -> i32 {
    if a >= b { a } else { b }
}

fn min(a: i32, b: i32) -> i32 {
    if a <= b { a } else { b }
}

fn prints(value: &str) {
    println!(" > as parameter: value: {value}");
}

fn orchestrator(value: &str, function: fn(&str) -> ()) {
    function(value)
}

fn function_pointer() {
    let n1 = r#"
    pod: Function Pointer Type
    - Refer to a function whose identity is not necessarily known at compile time
    - Points to executable code within memory
    ---"#;
    println!("{n1}");

    println!("Function Pointers");

    let mut function: fn(i32, i32) -> i32 = max;
    let r1 = function(15, 30);
    println!(" > max: r1: {r1}");

    function = min;
    let r2 = function(15, 30);
    println!(" > min: r2: {r2}");
}

fn function_as_parameter() {
    println!("Function Pointers");
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
