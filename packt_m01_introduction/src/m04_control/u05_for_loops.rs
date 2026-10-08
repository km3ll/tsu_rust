//! # For Loops and Their Variants

fn for_loops() {
    let n1 = r#"
    ---
    pod: For-Loops
    - We know the number of times a block will be executed
    - The values of vectors are consumed inside a for-loop
    - An iterator on a vector allows the borrowing of each of its elements
    ---"#;
    println!("{n1}");

    println!("For-Loop");
    let mut v1: Vec<i32> = vec![1, 2, 3, 4, 5];
    println!(" > v1: {v1:?}");

    println!(" > range");
    for i in 0..=4 {
        println!(" > i: {}", &v1[i]);
    }

    println!(" > vector");
    for e in &v1 {
        println!(" > e: {e}");
    }

    println!(" > iter");
    for e in v1.iter() {
        println!(" > ie: {e}");
    }

    println!(" > iter_mut");
    for e in v1.iter_mut() {
        println!(" > ie: {e}");
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
