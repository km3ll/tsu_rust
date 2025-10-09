fn start_game_v1() {
	let player1 = String::from("Player 1");
	let player2 = String::from("Player 2");

	let result = first_turn_two_lifetimes(player1.as_str(), player2.as_str());

	/**
	 * pod: Borrow checker
	 * - Knows that the lifetime of result is still valid
	 */
	println!("Player going first is: {}", result);
} // Lifetime of player1 and player2 ends here

fn start_game_v2() {
	let player1 = String::from("Player 1");
	{
		let player2 = String::from("Player 2");
		let result = first_turn_two_lifetimes(player1.as_str(), player2.as_str());
		println!("Player going first is: {}", result);
	}
}

fn start_game_v3() {
	let player1 = String::from("Player 1");
	let result: &str;
	{
		let player2 = String::from("Player 2");
		result = first_turn_one_lifetime(player1.as_str(), player2.as_str());
	}
	println!("Player going first is: {}", result);
}

fn start_game_v4() {
	let player1 = String::from("Player 1");
	let result1: &str;
	let result2: &str;
	{
		let player2 = String::from("Player 2");
		result1 = first_turn_static_lifetime_v1(player1.as_str(), player2.as_str());
		result2 = first_turn_static_lifetime_v2(player1.as_str(), player2.as_str());
	}
	println!("Player going first is: {}", result1);
	println!("Player going first is: {}", result2);
}

/**
 * pod: Lifetime specifiers
 * - A.k.a. generic lifetime annotations
 * - A way to describe the relationship between lifetimes of references
 * - Created with an apostrophe or tick '
 * - The convention is to define it as lowercase starting with 'a
 */

// error: missing lifetime specifier
// fn first_turn(p1: &str, p2: &str) -> &str {

/**
 * pod: Relationship between lifetimes
 * - The lifetime of the return value is equal to
 *   the shortest lifetime passed in
 */
fn first_turn_two_lifetimes<'a>(p1: &'a str, p2: &'a str) -> &'a str {
	if rand::random() {
		p1
	} else {
		p2
	}
}

/**
 * pod: Lifetimes of return values
 * - The lifetime of a return value must be tied to the lifetime
 *   of the input parameters
 */
fn first_turn_one_lifetime<'a>(p1: &'a str, p2: &str) -> &'a str {
	p1
}

/**
 * pod: Static lifetime
 * - A lifetime that lasts for the entire duration of the program
 * - One example are string slices
 * - They live in the program's binary
 * - The lifetime must be at least as long as p1 or p2
 */
fn first_turn_static_lifetime_v1<'a>(p1: &'a str, p2: &str) -> &'a str {
	let s: &'static str = "Hello, Ferris!";
	s
}

fn first_turn_static_lifetime_v2(p1: &str, p2: &str) -> &'static str {
	let s: &'static str = "Hello, Ferris!";
	s
}

fn generic_lifetimes_same_lifetimes() {
	println!("Same lifetimes");
	start_game_v1();
}

fn generic_lifetimes_different_lifetimes() {
	println!("Different lifetimes");
	start_game_v2();
}

fn generic_lifetimes_one_lifetime() {
	println!("One lifetime");
	start_game_v3();
}

fn generic_lifetimes_static_lifetime() {
	println!("Static lifetime");
	start_game_v4();
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn run_generic_lifetimes_same_lifetimes() {
		generic_lifetimes_same_lifetimes();
	}

	#[test]
	fn run_generic_lifetimes_different_lifetimes() {
		generic_lifetimes_different_lifetimes();
	}

	#[test]
	fn run_generic_lifetimes_static_lifetime() {
		generic_lifetimes_static_lifetime();
	}
}
