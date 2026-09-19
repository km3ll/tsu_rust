//! # Dereferencing

fn deref_stack() {
    let n1 = r#"
    pod: Dereferencing
    - The concept of accessing the value pointed to by a reference or pointer
    - The deref operation (*) can be used to change the value of the original variable
    - It is not logical to use a mutable reference to transfer ownership
    - Moving a value out of a mutable reference could potentially leave the reference invalid
    - A mutable reference can be copied only once, regardless of data being stack or heap allocated
    ---"#;
    println!("{n1}");

    println!("Dereferencing: Stack");
    let mut stack_data: i32 = 42;
    let ref1: &mut i32 = &mut stack_data;

    let dref1_copy = *ref1;
    println!(" > copy: {dref1_copy}");

    *ref1 = 100;
    println!(" > changed original: {stack_data}");
}

fn deref_heap() {
    println!("Dereferencing: Heap");
    let mut heap_data: Vec<i32> = vec![1, 3, 5];
    let ref1: &mut Vec<i32> = &mut heap_data;

    // Not allowed
    // let dref1_copy = *ref1;
    let dref1_cloned: Vec<i32> = ref1.clone();
    println!(" > cloned: {:?}", dref1_cloned);
}

fn deref_copy_once() {
    let n1 = r#"
    pod: Copy Once
    - Mutable references cannot be copied, but can only be moved
    - References, which are stored on the stack, are copied by default. However, mutable references is an exception
    - We cannot have multiple mutable references
    ---"#;
    println!("{n1}");

    println!("Dereferencing: mutable copy once");
    let mut heap_data: Vec<i32> = vec![1, 3, 5];
    let ref1: &mut Vec<i32> = &mut heap_data;

    let m1: &mut Vec<i32> = ref1;

    // cannot borrow `*ref1` as mutable more than once at a time [E0499]
    // second mutable borrow occurs here
    // let m2: &mut Vec<i32> = ref1;
    println!(" > mutable moved m1: {:?}", m1);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_deref_stack() {
        deref_stack()
    }

    #[test]
    fn run_deref_heap() {
        deref_heap()
    }

    #[test]
    fn run_deref_copy_once() {
        deref_copy_once()
    }
}
