pub fn variable_creation() {
	// pod: Immutability: variables are immutable by default
	let a1: i16 = 5;
	let a2: f32 = 5.0;
	println!("a1: {a1}");
	println!("a2: {a2}");
}

pub fn variable_mutability() {
	// pod: Mutability: add 'mut' keyword after 'let', you modify a single variable
	let mut m1: i16 = 4;
	println!("m1: {m1}");
	m1 = 6;
	println!("m1: {m1}");
}

pub fn variable_shadowing() {
	// pod: Shadowing: you create two separate variables
	let s1: i32 = 10;
	let s1: i32 = 20;
	println!("s1: {s1}");
}

pub fn variable_scope() {
	// pod: Scope: outer scope
	let d1: i16 = 40;
	{
		// pod: Scope: this varible lives within the scope of brackets {} (inner scope)
		let d1: i16 = 30;
		println!("inner d1: {d1}");
	}
	println!("outer d1: {d1}");
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn variable_creation_runs() {
		variable_creation();
		assert!(true)
	}

	#[test]
	fn variable_mutability_runs() {
		variable_mutability();
		assert!(true)
	}

	#[test]
	fn variable_scope_runs() {
		variable_scope();
		assert!(true)
	}
}
