//! # The match Control Flow Construct

use rand::Rng;

#[derive(Debug)]
enum CoinV1 {
	Penny,
	Nickel,
	Dime,
	Quarter,
}

#[derive(Debug)]
enum CoinV2 {
	Penny,
	Nickel,
	Dime,
	Quarter(UsState),
}

#[derive(Debug)]
enum UsState {
	Alabama,
	Alaska,
	Arizona,
}

fn value_in_cents_v1(coin: CoinV1) -> u8 {
	match coin {
		CoinV1::Penny => {
			println!("Match: lucky penny: {:?}", coin);
			1
		}
		CoinV1::Nickel => 5,
		CoinV1::Dime => 10,
		CoinV1::Quarter => 25,
	}
}

fn value_in_cents_v2(coin: CoinV2) -> u8 {
	match coin {
		CoinV2::Penny => {
			println!("Match: lucky penny: {:?}", coin);
			1
		}
		CoinV2::Nickel => 5,
		CoinV2::Dime => 10,
		CoinV2::Quarter(state) => {
			println!("Match: Quarter from state: {state:?}");
			25
		}
	}
}

fn plus_one(x: Option<i32>) -> Option<i32> {
	match x {
		Some(i) => Some(i + 1),
		None => None,
	}
}

fn check_v1(dice_roll: u8) {
	println!("Match: roll v1");
	match dice_roll {
		3 => println!("> add fancy hat"),
		7 => println!("> remove fancy hat"),
		other => println!("> move {other} spaces"),
	}
}

fn check_v2(dice_roll: u8) {
	println!("Match: roll v2");
	match dice_roll {
		3 => println!("> add fancy hat"),
		7 => println!("> remove fancy hat"),
		_ => println!("> roll again"),
	}
}

fn check_v3(dice_roll: u8) {
	println!("Match: roll v3");
	match dice_roll {
		3 => println!("> add fancy hat"),
		7 => println!("> remove fancy hat"),
		_ => (),
	}
}

fn match_definition() {
	let n1 = r#"
	pod: Match
	- Compares a value against a series of patterns
	- The compiler confirms that all possible cases are handled
	- The code associated with each arm is an expression
	---"#;
	println!("{n1}");

	let coin = CoinV1::Penny;
	value_in_cents_v1(coin);
}

fn match_binding() {
	let n1 = r#"
	pod: Match Arms
	- Have two parts: a pattern and some code
	- They can bind to the parts of the values that match the pattern
	- The catch-all pattern binds the pattern to a value
	- The underscore `_` pattern does not bind to any value
	- The unit value / empty tuple `()` does not run any code
	---"#;
	println!("{n1}");

	let coin = CoinV2::Quarter(UsState::Alabama);
	value_in_cents_v2(coin);
}

fn match_option() {
	let i1 = rand::thread_rng().gen_range(1..=100);
	let op1 = plus_one(Some(i1));
	let op2 = plus_one(None);
	println!("Match: op1: {op1:?}, op2: {op2:?}");
}

fn match_catch_all() {
	let r1: u8 = rand::thread_rng().gen_range(1..=6);
	check_v1(r1);

	let r2 = rand::thread_rng().gen_range(1..=6);
	check_v2(r2);

	let r3 = rand::thread_rng().gen_range(1..=6);
	check_v3(r3);
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn run_match_definition() {
		match_definition();
	}

	#[test]
	fn run_match_binding() {
		match_binding();
	}

	#[test]
	fn run_match_option() {
		match_option();
	}

	#[test]
	fn run_match_catch_all() {
		match_catch_all();
	}
}
