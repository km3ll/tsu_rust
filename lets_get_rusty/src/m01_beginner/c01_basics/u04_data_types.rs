//! # Data Types

pub fn types_boolean() {
    let n1 = r#"
    ---
	pod: Scalar Types
	- Store a single value
	- Boolean
	- Unsigned integers (positive)
	- Signed integers (positive or negative)
	- Single character is represented as char
	- All string literals as slices
	---"#;
    println!("{n1}");

    println!("Boolean");
    let b1: bool = true;
    println!(" > b1: {b1}")
}

pub fn types_unsigned_integers() {
    println!("Unsigned Integer");
    let ui1: u8 = 8;
    let ui2: u16 = 16;
    let ui3: u32 = 32;
    let ui4: u64 = 64;
    let ui5: u128 = 128;
    println!(" > ui5: {ui5}")
}

pub fn types_signed_integers() {
    println!("Signed Integer");
    let si1: i8 = -8;
    let si2: i16 = 16;
    let si3: i32 = -32;
    let si4: i64 = 64;
    let si5: i128 = -128;
    println!(" > si5: {si5}")
}

pub fn types_floating_point() {
    println!("Floating Point");
    let fp1: f32 = 1.0;
    let fp2: f64 = 2.0;
    println!(" > fp1: {fp1}")
}

pub fn types_platform_specific() {
    println!("Platform Specific");
    let usize: usize = 1;
    let isize: isize = 2;
    println!(" > usize: {usize}");
    println!(" > isize: {isize}");
}

pub fn types_chars_slices_strings() {
    println!("Types");
    let char1: char = 'c';
    let str1: &str = "Hello";
    let string1: String = String::from("Hello");
    println!(" > char: {char1}");
    println!(" > slice &str: {str1}");
    println!(" > String: {string1}");
}

pub fn types_arrays() {
    let n1 = r#"
    ---
	pod: Compound Types
	- Store multiple values
	- Arrays
	- Tuples
	---"#;
    println!("{n1}");

    println!("Array");
    let arr1: [i32; 5] = [1, 2, 3, 4, 5];
    let e0: i32 = arr1[0];
    println!(" > arr1: {arr1:?}");
    println!(" > element e0: {e0}");
}

pub fn types_tuples() {
    println!("Tuple");
    let tup1: (i32, f64, &str) = (5, 6.0, "7");
    let (e1, e2, e3) = tup1;
    println!(" > tuple: {tup1:?}");
    println!(" > element e1: {e1}");
}

pub fn types_unit_or_empty_tuple() {
    println!("Unit Type");
    let unit: () = ();
    println!(" > empty tuple: {unit:?}")
}

pub fn types_alias() {
    let n1 = r#"
    ---
	pod: Type Alias
	- A new name for an existing type
	---"#;
    println!("{n1}");

    println!("Type Alias");
    type Age = u8;
    let a1: Age = 40;
    println!(" > Age = u8: a1: {a1}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_types_boolean() {
        types_boolean();
    }

    #[test]
    fn run_types_unsigned_integers() {
        types_unsigned_integers();
    }

    #[test]
    fn run_types_signed_integers() {
        types_signed_integers();
    }

    #[test]
    fn run_types_floating_point() {
        types_floating_point();
    }

    #[test]
    fn run_types_platform_specific() {
        types_platform_specific();
    }

    #[test]
    fn run_types_chars_slices_strings() {
        types_chars_slices_strings();
    }

    #[test]
    fn run_types_arrays() {
        types_arrays();
    }

    #[test]
    fn run_types_tuples() {
        types_tuples();
    }

    #[test]
    fn run_types_unit_or_empty_tuple() {
        types_unit_or_empty_tuple();
    }

    #[test]
    fn run_types_alias() {
        types_alias();
    }
}
