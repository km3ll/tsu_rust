fn variables() {
    let n1 = r#"
	pod: Variable
	- A named storage that programs can manipulate
	- Identifiers to refer to memory allocations
	- Immutable by default (let / let mut)
	---
	pod: Data Type
    - Size and layout of the variable in memory
	- Range of values that it can store
	- Set of operations that can be performed
	---
	pod: Variable Naming Rules
	- Composed of letters, digits and underscore character
	- Must beging with letter or underscore
	- Names are case sensitive
	---
	pod: Rust
	- Statically typed programming language
	---"#;
    println!("{n1}");
}

fn scalar_types() {
    let n1 = r#"
	pod: Scalar Type
	- Represents a single value
	- Integer, float, boolen and character
	---
	pod: Integer
	- A number without a fraction component
	- Categorized as signed ("i") and unsigned ("u")
	- Bits used inside the memory to represent variables of these types: 8, 16, 32, and 64
	---
	pod: Float
	- Store numbers with decimal points
	---
	pod: Boolean
	- Has two possible values: true or false
	---
	pod: Character
	- Can represent a single letter, digit, emoji, unicode scalar value or special character
	- Always in single quotes
	---"#;
    println!("{n1}");
}

fn scalar_integers() {
    println!("Scalar: signed integers");
    println!(" > i8: min: {}, max: {}", i8::MIN, i8::MAX);
    println!(" > i16: min: {}, max: {}", i16::MIN, i16::MAX);
    println!(" > i32: min: {}, max: {}", i32::MIN, i32::MAX);
    println!(" > i64: min: {}, max: {}", i64::MIN, i64::MAX);

    println!("Scalar: unsigned integers");
    println!(" > u8: min: {}, max: {}", u8::MIN, u8::MAX);
    println!(" > u16: min: {}, max: {}", u16::MIN, u16::MAX);
    println!(" > u32: min: {}, max: {}", u32::MIN, u32::MAX);
    println!(" > u64: min: {}, max: {}", u64::MIN, u64::MAX);
}

fn scalar_floats() {
    println!("Scalar: floats");
    println!(" > f32: min: {}, max: {}", f32::MIN, f32::MAX);
    println!(" > f64: min: {}, max: {}", f64::MIN, f64::MAX);
}

fn scalar_booleans() {
    let is_active = true;
    let is_enabled = false;
    println!("Scalar: booleans: {:?}", (is_active, is_enabled));
}

fn scalar_characters() {
    let c1 = 'a';
    let c2 = '🦀';
    println!("Scalar: chars: c1: {}, c2: {}", c1, c2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_variables() {
        variables()
    }

    #[test]
    fn run_scalar_types() {
        scalar_types()
    }

    #[test]
    fn run_scalar_integers() {
        scalar_integers()
    }

    #[test]
    fn run_scalar_floats() {
        scalar_floats()
    }

    #[test]
    fn run_scalar_booleans() {
        scalar_booleans()
    }

    #[test]
    fn run_scalar_characters() {
        scalar_characters()
    }
}
