#![allow(dead_code, unused)]
use std::{ collections::HashMap, error::Error, io, num::ParseIntError };

#[derive(Debug)]
struct Card {
    number: u32,
    exp: Expiration,
    cvv: u32,
}

#[derive(Debug)]
struct Expiration {
    month: u32,
    year: u32,
}

#[derive(Debug)]
enum CreditCardError {
    InvalidInput(String), // Custom message
    Other(String, String) // Underlying error + custom message
}

pub fn find_credit_card(name: &str) {

    env_logger::init();

    let credit_cards = HashMap::from([
        ("Amy", "1234567 04 25 123"),
        ("Tim", "1234567 06 27"),      // Invalid record
        ("Bob", "1234567 Dec 27 123"), // Invalid record
    ]);

    let result = get_credit_card_info(&credit_cards, name.trim());
        
    match result {
        Ok(card) => println!("\nCredit card info: {card:?}"),
        Err(err) => {

            // When we match error we are taking ownership of 'msg'
            // To fix this, instead of matching against the error, we match against
            // a reference to error.

            match &err {
                CreditCardError::InvalidInput(msg) => println!("{msg}"),
                CreditCardError::Other(_, _) => println!("\nSomething went wrong. Try again."),
            }

            log::error!("\n{err:?}");
        }
    }

}

fn get_credit_card_info(
    credit_cards: &HashMap<&str, &str>,
    name: &str,
) -> Result<Card, CreditCardError> {

    let card_string = credit_cards.get(name).ok_or(
        CreditCardError::InvalidInput(format!("No credit card was found for {name}."))
    )?;

    let card = parse_card(card_string)
    .map_err(|e| {
        CreditCardError::Other(e, format!("{name}'s card could not be parsed."))
    })?;
    Ok(card)

}

fn parse_card(card: &str) -> Result<Card, String> {

    let mut numbers: Vec<u32> = parse_card_numbers(card)
        .map_err(|e| { e.to_string() })?;

    let len = numbers.len();
    let expected_len = 4;

    if len != expected_len {
        return Err(format!(
            "Incorrect number of elements parsed. Expected {expected_len} but got {len}. Elements: {numbers:?}" // debug formatting
        ))
    }

    let cvv: u32 = numbers.pop().unwrap();
    let year: u32 = numbers.pop().unwrap();
    let month: u32 = numbers.pop().unwrap();
    let number: u32 = numbers.pop().unwrap();

    Ok( Card { number, exp: Expiration { month, year },  cvv } )

}

fn parse_card_numbers(card: &str) -> Result<Vec<u32>, ParseIntError> {

    let numbers = card
        .split(" ")
        .into_iter()
        .map(|s| {
            s.parse()
        })
        .collect::<Result<Vec<u32>, _>>()?;
    Ok(numbers)

}