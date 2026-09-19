//! # Closures - Part 2

fn closures_immutable_ref() {
    let n1 = r#"
    pod: Closures
    - Can capture variables from environment not specified in its signature
    - When out of scope (after their execution) their references end
    ---"#;
    println!("{n1}");

    let vec1 = vec![1, 2, 3];
    let closure = || println!(" > immutable vec1: {vec1:?}");

    println!("Closures");
    closure()
}

fn closure_mutable_ref() {
    let mut vec2 = vec![1, 2, 3];
    let mut closure = || {
        vec2.push(100);
    };

    closure();
    println!("Closures");
    println!(" > mutable vec2: {vec2:?}");
}

fn closure_moved_ownership() {
    let vec3 = vec![1, 2, 3];
    let closure = || {
        println!("> moved vec3: {vec3:?}");
        let vec4 = vec3;
        println!("> moved vec4: {vec4:?}");
    };

    println!("Closures");
    closure();
    // println!(" > moved vec3: {vec3:?}"); // Value used after being moved
    // println!(" > moved vec4: {vec4:?}"); // Cannot find value `vec4
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
