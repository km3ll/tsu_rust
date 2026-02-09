//! # Custom Errors Part 2

#![allow(dead_code)]
use core::num;
use std::{collections::HashMap, fmt::format, io, num::ParseIntError};

#[derive(Debug)]
enum CreditCardError {
    InvalidInput(String),
    Other(String, String),
}

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

fn parse_card_numbers(card: &str) -> Result<Vec<u32>, ParseIntError> {
    let numbers: Vec<u32> = card
        .split(" ")
        .into_iter()
        .map(|value| value.parse())
        .collect::<Result<Vec<u32>, _>>()?;
    Ok(numbers)
}

fn parse_card(card: &str) -> Result<Card, String> {
    let mut numbers = parse_card_numbers(card).map_err(|e| e.to_string())?;

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

fn get_credit_card_info(
    credit_cards: &HashMap<&str, &str>,
    name: &str,
) -> Result<Card, CreditCardError> {
    let card_string: &&str =
        credit_cards
            .get(name)
            .ok_or(CreditCardError::InvalidInput(format!(
                "No credit card found for name: {name}."
            )))?;

    let card: Card = parse_card(card_string)
        .map_err(|e| CreditCardError::Other(e, format!("{name}'s card could not be parsed.")))?;
    Ok(card)
}

fn cli_app() {
    env_logger::init();

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

    let result: Result<Card, CreditCardError> = get_credit_card_info(&credit_cards, &name.trim());

    match result {
        Ok(card) => println!("Credit card info: {:?}", card),
        Err(error) => {
            match &error {
                CreditCardError::InvalidInput(msg) => println!("{msg}"),
                // pod: ownership of error could be taken here.
                CreditCardError::Other(_, _) => println!("Something went wrong. Try again."),
            }
            // pod: Error logging
            log::error!("{error:?}")
        }
    };
}

fn custom_errors_2_cli_app() {
    println!("custom error cli-app");
    cli_app();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_custom_errors_2_cli_app() {
        //custom_errors_2_cli_app();
    }
}
