pub fn const_constants() {
	/**
	 * pod: Constants 
	 * - Computed at compile time
	 * - The value of the constants is inline (replaced)
	 * - They do not ocupy a location in memory
	 * - Their naming convention is screaming snake-case
	 */
	const MAX_PLAYERS: u8 = 10;
	println!("Max players: {MAX_PLAYERS}");
}

pub fn conts_static() {
	/**
	 * pod: Static
	 * - Can be mutable, but it's unsafe.
	 * - Statics ocupy space in memory
	 * - There is one instance of the value
	 */
	static CASINO_NAME: &str = "Rusty Casino";
	println!("Casino name: {:?}", CASINO_NAME);
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn run_const_constants() {
		const_constants();
	}

	#[test]
	fn run_conts_static() {
		conts_static();
	}
}
