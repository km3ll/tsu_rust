pub fn const_constants() {
	/***
	 * pod: Constants are computed at compile time
	 * The value of the constants is inline (replaced)
	 * They do not ocupy a location in memory
	 * Their naming convention is screaming snake-case
	 */
	const MAX_PLAYERS: u8 = 10;
	println!("Max players: {MAX_PLAYERS}");
}

pub fn conts_static() {
	/***
	 * pod: Static can be mutable, but it's unsafe.
	 * Statics ocupy space in memory
	 * There is one instance of the value
	 */
	static CASINO_NAME: &str = "Rusty Casino";
	println!("Casino name: {:?}", CASINO_NAME);
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn const_constants_runs() {
		const_constants();
		assert!(true)
	}

	#[test]
	fn conts_static_runs() {
		conts_static();
		assert!(true)
	}
}
