//! # Closures - Part 1

fn closures() {
    let n1 = r#"
    pod: Closures
    - Anonymous functions
    - Can be assigned to variables
    - They capture their environment (variables)
    - The ownership rules also apply to closures
    - Can infer the types of inputs and outputs
    - Can be passed as parameter to other functions
    ---"#;
    println!("{n1}");

    let x: i32 = 25;
    let closure = || println!(" > square: {}", x * x);
    println!("Closures");
    println!(" > x: {x}");
    closure();
}

fn closures_inputs() {
    let x = 11;
    let y = 23;
    let closure = |x: i32| println!(" > square of {} is {}", x, x * x);
    println!("Closures");
    closure(x);
    closure(y);
}

fn closures_inferred_type() {
    let x = 11;
    let closure = |x| println!(" > square of {} is {}", x, x * x);
    println!("Closures");
    closure(x);
}

fn comparator<F: Fn(i32, i32) -> bool>(x: i32, y: i32, compare: F) -> bool {
    compare(x, y)
}

fn closures_as_parameter() {
    let x = 55;
    let y = 10;
    let first_is_greater = |a: i32, b: i32| a > b;
    let result = comparator(x, y, first_is_greater);
    println!("Closures");
    println!(" > x: {x}, y: {y}, predicate result: {result}");
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
