//! # Compound Data Types - Tuples, Arrays

fn tuples() {
    let n1 = r#"
    pod: Tuples
    - Have fixed length
    - Destructuring allows the unpacking of values inside a tuple
    - Allow nested tuples
    - Allow an empty tuple
    ---"#;
    println!("{n1}");

    let t1: (&str, i32) = ("age", 30);
    println!("Tuples: t1: {:?}", t1);

    println!(" > 0: {}", t1.0);
    println!(" > 1: {}", t1.1);

    let (key, value) = t1;
    println!(" > key: {key}");
    println!(" > value: {value}");
}

fn arrays() {
    let n1 = r#"
    pod: Arrays
    - Collections of elements of the same type
    - Stored in contiguos memory locations
    - Size must be known at compile time
    - The first index number is Zero (0)
    ---
    pod: Array Slices
    - A subset of references to elements in the array
    - The starting index is the first position in the slice
    - The ending index is one more than the last position in the slice
    - To include the last element also use the equal(=) sign as in [0..=3]
    ---"#;
    println!("{n1}");

    let a1: [i16; 5] = [8, 3, 1, 7, 9];
    println!("Arrays: a1: {:#?}", a1);

    println!(" > first: {}", a1[0]);
    println!(" > last: {}", a1[4]);
    println!(" > length: {}", a1.len());
    println!(" > size in memory bytes: {}", std::mem::size_of_val(&a1));
}

fn arrays_mutable() {
    println!("Arrays: mutable");
    let mut a2 = [6, 7, 8, 9, 10];
    println!(" > before: {:?}", a2);
    a2[4] = 99;
    println!(" > after: {:?}", a2);
}

fn arrays_initialization() {
    let a3 = [true; 3];
    println!("Arrays: initialized: {:#?}", a3);
}

fn arrays_slices() {
    let a4: [i32; 10] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    let s1: &[i32] = &a4[0..3];
    println!("Arrays: slice 0..3: {:?}", s1);

    let s2 = &a4[0..=3];
    println!("Arrays: slice 0..=3: {:?}", s2);
}

fn arrays_get_option() {
    let a5: [&str; 3] = ["oranges", "apples", "strawberries"];
    let v1: Option<&&str> = a5.get(5);
    let v2: Option<&&str> = a5.get(1);
    println!("Arrays: get()");
    println!(" > v1: {:?}", v1);
    println!(" > v2: {:?}", v2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_tuples() {
        tuples()
    }

    #[test]
    fn run_arrays() {
        arrays()
    }

    #[test]
    fn run_arrays_mutable() {
        arrays_mutable()
    }

    #[test]
    fn run_arrays_initialization() {
        arrays_initialization()
    }

    #[test]
    fn run_arrays_slices() {
        arrays_slices()
    }

    #[test]
    fn run_arrays_get_option() {
        arrays_get_option()
    }
}
