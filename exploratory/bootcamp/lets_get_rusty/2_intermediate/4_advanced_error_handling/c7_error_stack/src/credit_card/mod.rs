use std::{collections::HashMap, error::{self, Error}, fmt, io, num::ParseIntError};
use std::fmt::Formatter;
use error_stack::{IntoReport, Report, Result, ResultExt};

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
struct ParsePaymentInfoError;
impl Error for ParsePaymentInfoError {}
impl fmt::Display for ParsePaymentInfoError {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        fmt.write_str("Parsing payment error: invalid payment info.")
    }
}

#[derive(Debug)]
enum CreditCardError {
    InvalidInput(String),
    Other
}
impl Error for CreditCardError {}
impl fmt::Display for CreditCardError {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        fmt.write_str("Credit card error: Could not retrieve credit card.")
    }
}

pub fn find_credit_card(name: &str) {

    let credit_cards = HashMap::from([
        ("Amy", "1234567 04 25 123"),
        ("Tim", "1234567 06 27"),      // Invalid record
        ("Bob", "1234567 Dec 27 123"), // Invalid record
    ]);

    let result = get_credit_card_info(&credit_cards, name.trim());
        
    match result {
        Ok(card) => println!("\nCredit card info: {card:?}"),
        Err(err) => {
            match err.current_context() {
                CreditCardError::InvalidInput(msg) => println!("\n{msg}"),
                CreditCardError::Other => println!("\nSomething went wrong. Try again.")
            }
            log::error!("\n{err:?}");
        }
    }

}

fn get_credit_card_info(
    credit_cards: &HashMap<&str, &str>,
    name: &str,
) -> Result<Card, CreditCardError> {

    let card_string = credit_cards.get(name).ok_or_else(|| {
        let msg = format!("No credit card was found for {name}.");
        Report::new(CreditCardError::InvalidInput(msg.clone()))
            .attach_printable(msg.clone())
    })?;

    let card = parse_card(card_string)
        .change_context(CreditCardError::Other)
        .attach_printable(format!("{name}'s card could not be parsed"))?;
    Ok(card)

}

fn parse_card(card: &str) -> Result<Card, ParsePaymentInfoError> {

    let mut numbers: Vec<u32> = parse_card_numbers(card)?;

    let len = numbers.len();
    let expected_len = 4;

    if len != expected_len {
        return Err(Report::new(ParsePaymentInfoError)
            .attach_printable(
                format!("Incorrect number of elements parsed. Expected {expected_len} but got {len}. Elements: {numbers:?}")
            ));
    }

    let cvv: u32 = numbers.pop().unwrap();
    let year: u32 = numbers.pop().unwrap();
    let month: u32 = numbers.pop().unwrap();
    let number: u32 = numbers.pop().unwrap();

    Ok( Card { number, exp: Expiration { month, year },  cvv } )

}

fn parse_card_numbers(card: &str) -> Result<Vec<u32>, ParsePaymentInfoError> {

    let numbers = card
        .split(" ")
        .into_iter()
        .map(|s| {
            s.parse()
            .report()
            .attach_printable_lazy(|| {
                format!("{s:?} could not be parsed as u32")
            })
        })
        .collect::<Result<Vec<u32>, _>>()
        .change_context(ParsePaymentInfoError)
        .attach_printable(format!(
            "Failed to parse input as numbers: input: {card}"
        ))?;

    Ok(numbers)

}