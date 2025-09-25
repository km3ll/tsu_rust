/**
 * pod: Scalar data types
 * - store a single value
 */

pub fn types_boolean() {
	let b1: bool = true;
	let b2: bool = false;
	println!("b1: {b1}");
	println!("b2: {b2}");
}

pub fn types_unsigned_integers() {
	/**
	 * pod: Unsigned integers must be positive
	 */
	let ui1: u8 = 8;
	let ui2: u16 = 16;
	let ui3: u32 = 32;
	let ui4: u64 = 64;
	let ui5: u128 = 128;
	println!("ui1: {ui1}");
	println!("ui2: {ui2}");
	println!("ui3: {ui3}");
	println!("ui4: {ui4}");
	println!("ui5: {ui5}");
}

pub fn types_signed_integers() {
	/**
	 * pod: Signed integers could be positive or negative
	 */
	let si1: i8 = -8;
	let si2: i16 = 16;
	let si3: i32 = -32;
	let si4: i64 = 64;
	let si5: i128 = -128;
	println!("si1: {si1}");
	println!("si2: {si2}");
	println!("si3: {si3}");
	println!("si4: {si4}");
	println!("si5: {si5}");
}

pub fn types_floating_point() {
	let fp1: f32 = 1.0;
	let fp2: f64 = 2.0;
	println!("fp1: {fp1}");
	println!("fp2: {fp2}");
}

pub fn types_platform_specific() {
	let ps1: usize = 1;
	let ps2: isize = 2;
	println!("ps1: {ps1}");
	println!("ps2: {ps2}");
}

pub fn types_chars_slices_strings() {
	/**
	 * pod: Single character is represented as char
	 */
	let c1: char = 'c';
	/**
	 * pod: All string literals as slices
	 */
	let s2: &str = "Hello";
	let s3: String = String::from("Hello");
	println!("c1: {c1}");
	println!("s2: {s2}");
	println!("s3: {s3}");
}

/**
 * pod: Compound data types 
 * - store multiple values
 */
pub fn types_arrays() {
	let arr1: [i32; 5] = [1, 2, 3, 4, 5];
	let e0: i32 = arr1[0];
	let e4: i32 = arr1[4];
	println!("arr1: {:?}", arr1);
	println!("e0: {e0}");
	println!("e4: {e4}");
}

pub fn types_tuples() {
	let tup1: (i32, f64, &str) = (5, 6.0, "7");
	println!("tup1: {:?}", tup1);
	println!("tup1_0: {}", tup1.0);
	println!("tup1_1: {}", tup1.1);
	println!("tup1_2: {}", tup1.2);
	/**
	 * pod: Destructuring a tuple
	 */
	let (te1, te2, te3) = tup1;
	println!("te1: {te1}");
	println!("te2: {te2}");
	println!("te3: {te3}");
}

pub fn types_unit_or_empty_tuple() {
	let unit: () = ();
	println!("unit: {:?}", unit);
}

pub fn types_alias() {
	/**
	 * pod: Type alias is a new name for an existing type
	 */
	type Age = u8;
	let a1: Age = 40;
	println!("a1: {a1}");
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
