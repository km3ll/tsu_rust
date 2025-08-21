pub fn constants() {
	println!("----------");
	println!("Constants");

	// Constants are computed at compile time
	// The value of the constants is inline (replaced)
	// They do not ocupy a location in memory
	const MAX_PLAYERS: u8 = 10;
	println!("Max players: {MAX_PLAYERS}");

	// Static can be mutable, but it's unsafe
	// Statics do ocupy space in memory.
	// There is only one instance of the value
	static CASINO_NAME: &str = "Rusty Casino";
	println!("Casino name: {:?}", CASINO_NAME);
}
