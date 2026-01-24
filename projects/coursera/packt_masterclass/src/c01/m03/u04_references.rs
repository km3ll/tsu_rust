//! # Mutable and Immutable References

fn references_rules() {
    let n1 = r#"
    pod: References Rules
    - One mutable reference in a scope
    - Many immutable references
    - Mutable and immutable cannot coexist
    - Data should not change when immutable references are in scope
    ---
    pod: Data Race
    - A condition that occurs when two or more threads try to access the same data at the same time
    ---"#;
    println!("{n1}");

    // 1st Rule
    let mut heap_v1 = vec![1, 2, 3];
    let r1v1: &mut Vec<i32> = &mut heap_v1;
    // let r2v1: &mut Vec<i32> = &mut heap_v1; Error
    // println!("Ownership Rule #1: {:?}, {:?}", r1v1, r2v1);

    // 2nd Rule
    let mut heap_v2 = vec![4, 5, 6];
    let r1v2: &Vec<i32> = &heap_v2;
    let r2v2: &Vec<i32> = &heap_v2;
    println!("Ownership Rule #2: {:?}, {:?}", r1v2, r2v2);

    // 2nd Rule
    let mut heap_v2 = vec![4, 5, 6];
    let r1v2: &Vec<i32> = &heap_v2;
    let r2v2: &Vec<i32> = &heap_v2;
    println!("Ownership Rule #2: {:?}, {:?}", r1v2, r2v2);

    // 3rd Rule
    let mut heap_v3 = vec![7, 8, 9];
    let r1v3: &Vec<i32> = &heap_v3;
    // let r2v3: &Vec<i32> = &mut heap_v3; Error
    // println!("Ownership Rule #2: {:?}, {:?}", r1v3, r2v3);
}

fn references_scope() {
    let n1 = r#"
    pod: Reference Scope
    - When the reference is first and last used
    ---"#;
    println!("{n1}");

    let mut heap_v4 = vec![10, 11, 12];
    let r1: &Vec<i32> = &heap_v4;
    let r2: &Vec<i32> = &heap_v4;
    println!("Reference scope: immutable before: {:?}, {:?}", r1, r2);
    // Scope of immutable r1 and r2 ends here

    // Mutable r3 is allowed here
    let r3: &mut Vec<i32> = &mut heap_v4;
    println!("Reference scope: mutable after: {:?}", r3);
    // Scope of mutable r3 ends here
}

fn references_immutable() {
    let heap_v5: Vec<i32> = vec![13, 14, 15];
    let r1: &Vec<i32> = &heap_v5;
    let r2: &Vec<i32> = &heap_v5;

    // Error: cannot borrow as mutable because it is also borrowed as immutable
    // heap_v5.push(16);
    println!("Reference mutable & immutable: {:?}, {:?}", r1, r2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_references_rules() {
        references_rules()
    }

    #[test]
    fn run_references_scope() {
        references_scope()
    }

    #[test]
    fn run_references_immutable() {
        references_immutable()
    }
}
