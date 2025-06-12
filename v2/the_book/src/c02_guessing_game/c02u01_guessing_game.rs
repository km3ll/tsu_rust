/***
pod: Prelude
A set of items defined in the standard library 'std' that it brings
into the scope of every program.
 */
use std::io;

pub fn start_game() {
	println!("Guess the number!");
	println!("Please, input your guess:");

	/***
	pod: Mutability
	In Rust, variables are immutable by default.
	Add 'mut' before a variable name to make it mutable
	 */

	/***
	pod: Associated Function
	The :: syntax in the ::new line indicates that new is an associated function
	of the String type. An associated function is a function that’s implemented
	on a type, in this case String.
	 */
	let mut guess: String = String::new();

	io::stdin()
		.read_line(&mut guess)
		.expect("Failed to read line");

	println!("You guessed: {}", guess);
}
