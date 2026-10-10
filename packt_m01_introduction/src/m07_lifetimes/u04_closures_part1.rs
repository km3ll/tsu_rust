//! # Closures - Part 1

fn closures() {
    let n1 = r#"
    ---
    pod: Closures
    - Anonymous functions
    - Can be assigned to variables
    - They capture their environment (variables)
    - The ownership rules also apply to closures
    - Can infer the types of inputs and outputs
    - Can be passed as parameter to other functions
    ---"#;
    println!("{n1}");

    println!("Closures");
    let x: i32 = 25;
    let closure = || println!(" > square: {}", x * x);
    println!(" > x: {x}");
    closure();
}

fn closures_inputs() {
    println!("Closures");
    let x = 11;
    let closure = |x: i32| println!(" > square of {} is {}", x, x * x);
    println!(" > input x: {x}");
    closure(x);
}

fn closures_inferred_type() {
    println!("Closures");
    let x = 20;
    let closure = |x| println!(" > square of {} is {}", x, x * x);
    println!(" > inferred type of x: {x}");
    closure(x);
}

fn comparator<F: Fn(i32, i32) -> bool>(x: i32, y: i32, compare: F) -> bool {
    compare(x, y)
}

fn closures_as_parameter() {
    println!("Closures");
    println!(" > as parameter type: comparator<F: Fn(i32, i32) -> bool>");
    let x = 55;
    let y = 10;
    let first_is_greater = |a: i32, b: i32| a > b;
    println!(" > x: {x}, y: {y}, comparator: first_is_greater()");
    let result = comparator(x, y, first_is_greater);
    println!(" > result: {result}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_closures() {
        closures()
    }

    #[test]
    fn run_closures_inputs() {
        closures_inputs()
    }

    #[test]
    fn run_closures_inferred_type() {
        closures_inferred_type()
    }

    #[test]
    fn run_closures_as_parameter() {
        closures_as_parameter()
    }
}
