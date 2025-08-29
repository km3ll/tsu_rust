pub fn types_() {
    println!("Base")
}

pub fn types_boolean() {
	let b1: bool = true;
	let b2: bool = false;
	println!("b1: {b1}");
	println!("b2: {b2}");
}

pub fn types_unsigned_integers() {
    // pod: Unsigned integers must be positive numbers
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn _runs() {
        types_();
        assert!(true)
    }

    #[test]
    fn types_boolean_runs() {
        types_boolean();
        assert!(true)
    }

    #[test]
    fn unsigned_integers_runs() {
        types_unsigned_integers();
        assert!(true)
    }

    #[test]
    fn signed_integers_runs() {
        types_signed_integers();
        assert!(true)
    }
    
}