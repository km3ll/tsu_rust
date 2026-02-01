//! # For Loops and Their Variants

fn for_loops() {
    let n1 = r#"
    pod: For-Loops
    - We know the number of times a block will be executed
    - The values of vectors are consumed inside a for-loop
    - An iterator on a vector allows the borrowing of each of its elements
    ---"#;
    println!("{n1}");

    let mut v1: Vec<i32> = vec![1, 2, 3, 4, 5];

    println!("For-Loop: range");
    for i in 0..=4 {
        println!(" > i: {}", &v1[i]);
    }

    println!("For-Loop: vector");
    for e in &v1 {
        println!(" > e: {e}");
    }

    println!("For-Loop: iter");
    for e in v1.iter() {
        println!(" > e: {e}");
    }

    println!("For-loop: iter_mut");
    for e in v1.iter_mut() {
        println!(" > e: {e}");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_for_loops() {
        for_loops()
    }
}
