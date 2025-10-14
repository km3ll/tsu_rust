#![allow(dead_code)]
use core::num;
use std::{collections::HashMap, io, num::ParseIntError};

/**
 * pod: Error improvements
 * - We can't react programatically to different error cases, as distinguishing
 *   between user errors vs internal errors
 * - Users should get a friendly error message if their input is invalid,
 *   because that's something they can fix.
 * - There is no separation between user-level reporting and dev-level reporting
 * - Devs should see technical details because Devs could fix them
 * - As Devs, not only do we want to see details of errors,
 *   but also we want to see an error chain, meaning the top-level error
 *   plus all the errors that caused it.
 */
#[derive(Debug)]
struct Expiration {
	year: u32,
	month: u32,
}

#[derive(Debug)]
struct Card {
	number: u32,
	exp: Expiration,
	cvv: u32,
}

/**
 * pod: v1 using unwrap() method
 */
fn parse_card_numbers_v1(card: &str) -> Vec<u32> {
	let numbers: Vec<u32> = card
		.split(" ")
		.into_iter()
		.map(|value| value.parse())
		.collect::<Result<Vec<u32>, _>>()
		.unwrap();
	numbers
}

/**
 * pod: v2 using question mark (?) operator
 */
fn parse_card_numbers_v2(card: &str) -> Result<Vec<u32>, ParseIntError> {
	let numbers: Vec<u32> = card
		.split(" ")
		.into_iter()
		.map(|value| value.parse())
		.collect::<Result<Vec<u32>, _>>()?;
	Ok(numbers)
}

/**
 * pod: v1 using unwrap() method
 */
fn parse_card_v1(card: &str) -> Card {
	let mut numbers = parse_card_numbers_v1(card);

	// pod: .pop() method to get the last element from a vector
	let cvv = numbers.pop().unwrap();
	let year = numbers.pop().unwrap();
	let month = numbers.pop().unwrap();
	let number = numbers.pop().unwrap();

	Card {
		number,
		exp: Expiration { year, month },
		cvv,
	}
}

/**
 * pod: v2 using question mark (?) operator
 */
fn parse_card_v2(card: &str) -> Result<Card, String> {
	let mut numbers = parse_card_numbers_v2(card).map_err(|e| e.to_string())?;

	let len: usize = numbers.len();
	let expected_len: usize = 4;

	if len != expected_len {
		return Err(format!(
			"Incorrect number of elements. Expected: {expected_len}, found: {len}. Elements: {numbers:?}"
		));
	}

	// pod: .pop() method to get the last element from a vector
	let cvv = numbers.pop().unwrap();
	let year = numbers.pop().unwrap();
	let month = numbers.pop().unwrap();
	let number = numbers.pop().unwrap();

	let card = Card {
		number,
		exp: Expiration { year, month },
		cvv,
	};
	Ok(card)
}

/**
 * pod: v1 using unwrap() method
 */
fn get_credit_card_info_v1(credit_cards: &HashMap<&str, &str>, name: &str) -> Card {
	let card_string = credit_cards.get(name).unwrap();
	let card = parse_card_v1(card_string);
	card
}

/**
 * pod: v2 using question mark (?) operator
 */
fn get_credit_card_info_v2(credit_cards: &HashMap<&str, &str>, name: &str) -> Result<Card, String> {
	let card_string: &&str = credit_cards
		.get(name)
		.ok_or(format!("No credit card found for name: {name}"))?;

	let card: Card = parse_card_v2(card_string)?;
	Ok(card)
}

/**
 * pod: v1 using unwrap() method
 */
fn cli_app_v1() {
	let credit_cards: HashMap<&str, &str> = HashMap::from([
		("Amy", "1234567 04 25 123"),
		("Tim", "1234567 06 27"),
		("Bob", "1234567 Dec 27 123"),
	]);

	println!("Enter your name: ");
	let mut name = String::new();

	io::stdin()
		.read_line(&mut name)
		.expect("Failed to read name");

	let result: Card = get_credit_card_info_v1(&credit_cards, &name.trim());

	println!("Credit card info: {:?}", result)
}

/**
 * pod: v2 using question mark (?) operator
 */
fn cli_app_v2() {
	let credit_cards: HashMap<&str, &str> = HashMap::from([
		("Amy", "1234567 04 25 123"),
		("Tim", "1234567 06 27"),
		("Bob", "1234567 Dec 27 123"),
	]);

	println!("Enter your name: ");
	let mut name = String::new();

	io::stdin()
		.read_line(&mut name)
		.expect("Failed to read name");

	let result: Result<Card, String> = get_credit_card_info_v2(&credit_cards, &name.trim());

	match result {
		Ok(card) => println!("Credit card info: {:?}", card),
		Err(error) => println!("{error}"),
	};
}

fn error_handling_cli_app_v1() {
	println!("cli-app v1");
	//cli_app_v1();
}

fn error_handling_cli_app_v2() {
	println!("cli-app v2");
	//cli_app_v2();
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn run_error_handling_cli_app_v1() {
		error_handling_cli_app_v1();
	}

	#[test]
	fn run_error_handling_cli_app_v2() {
		error_handling_cli_app_v2();
	}
}
