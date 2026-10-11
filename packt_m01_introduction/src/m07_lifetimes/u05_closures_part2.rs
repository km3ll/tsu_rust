//! # Closures - Part 2

fn closures_immutable_ref() {
    let n1 = r#"
    ---
    pod: Closures
    - Can capture variables from environment not specified in its signature
    - When out of scope (after their execution) their references end
    ---"#;
    println!("{n1}");

    println!("Closures");

    let vec1 = vec![1, 2, 3];
    println!(" > outer: vec1: {vec1:?}");

    let closure = || println!(" > inner: vec1: {vec1:?}");
    closure()
}

fn closure_mutable_ref() {
    println!("Closures");

    let mut vec2 = vec![1, 2, 3];
    println!(" > outer: mut vec2: {vec2:?}");
    let mut closure = || {
        println!(" > inner: push: 100");
        vec2.push(100);
    };

    closure();
    println!(" > outer: mut vec2: {vec2:?}");
}

fn closure_moved_ownership() {
    println!("Closures");

    let vec3 = vec![1, 2, 3];
    println!(" > outer: vec3: {vec3:?}");

    let closure = || {
        let vec4 = vec3;
        println!(" > inner: vec3 moved to new vec4: {vec4:?}");
    };

    closure();
    // println!(" > moved vec3: {vec3:?}"); // Value used after being moved
    // println!(" > moved vec4: {vec4:?}"); // Cannot find value `vec4
    println!(" > outer: error: vec3 used after being moved");
    println!(" > outer: error: cannot find value 'vec4'");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_closure_mutable_ref() {
        closure_mutable_ref()
    }

    #[test]
    fn run_closures_immutable_ref() {
        closures_immutable_ref()
    }

    #[test]
    fn run_closure_moved_ownership() {
        closure_moved_ownership()
    }
}
