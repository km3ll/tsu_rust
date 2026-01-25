pub fn const_constants() {
	let n1 = r#"
	pod: Constants
	- Computed at compile time
	- The value of the constants is inline (replaced)
	- They do not occupy a location in memory
	- Their naming convention is screaming snake-case
	---"#;
	println!("{n1}");

	const MAX_PLAYERS: u8 = 10;
	println!("const MAX_PLAYERS: {MAX_PLAYERS}")
}

pub fn const_static() {
	let n1 = r#"
	pod: Static
	- Can be mutable, but it's unsafe
	- Statics occupy space in memory
	- There is one instance of the value
	---"#;
	println!("{n1}");

	static CASINO_NAME: &str = "Rusty Casino";
	println!("static CASINO_NAME: {CASINO_NAME}")
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn run_const_constants() {
		const_constants();
	}

	#[test]
	fn run_const_static() {
		const_static();
	}
}
