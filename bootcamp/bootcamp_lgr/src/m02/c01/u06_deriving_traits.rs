//! # Deriving Traits

#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

fn deriving_traits_debug() {
    let n1 = r#"
    pod: trait: Debug
    - Uses `{:?}` syntax to print a type with debug formatting
    - Only works if the type implements the Debug trait
    - Can be derived using `#[derive(Debug)]`
    ---"#;
    println!("{n1}");
    let p1 = Point { x: 3, y: 1 };
    println!("p1: {:?}", p1)
}

fn deriving_traits_partial_eq() {
    let n1 = r#"
    pod: trait: PartialEq
    - Can be derived to compare with the `==` operator
    ---"#;
    println!("{n1}");
    let p1 = Point { x: 3, y: 1 };
    let p2 = Point { x: 3, y: 1 };
    let res = p2 == p2;

    println!("p1 == p2: {}", res);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_deriving_traits_debug() {
        deriving_traits_debug();
    }

    #[test]
    fn run_deriving_traits_partial_eq() {
        deriving_traits_partial_eq();
    }
}
