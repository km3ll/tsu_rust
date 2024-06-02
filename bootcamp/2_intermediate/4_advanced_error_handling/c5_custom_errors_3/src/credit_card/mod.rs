#![allow(dead_code, unused)]
use std::{ 
    collections::HashMap, 
    error::Error, 
    fmt,
    io, 
    num::ParseIntError 
};

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

enum CreditCardError {
    InvalidInput(String), // Custom message
    Other(Box<dyn Error>, String) // Underlying error + custom message
}

impl fmt::Debug for CreditCardError {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CreditCardError::InvalidInput(msg) => write!(fmt, "{self}\n{msg}"),
            CreditCardError::Other(err, msg) => write!(fmt, "{self}\n{msg}\n\nCaused by:\n\t{err:?}"),
        }
    }
}

struct ParsePaymenInfoError {
    source: Option<Box<dyn Error>>,
    msg: String
}

impl fmt::Debug for ParsePaymenInfoError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self}\n\t{}", self.msg)?;
        if let Some(e) = self.source.as_ref() {
            write!(f, "\n\nCaused by:\n\t{e:?}")?;
        }
        Ok(())
    }
}

// The From trait is used to convert a value of one type to a value of a different type
impl From<ParseIntError> for ParsePaymenInfoError {
    fn from(e: ParseIntError) -> Self {
        ParsePaymenInfoError {
            source: Some(Box::new(e)),
            msg: String::new()
        }
    }
}

impl Error for ParsePaymenInfoError {
    fn source(&self) -> Option<&(dyn Error +'static)> {
        self.source.as_deref()
    }
}

impl fmt::Display for ParsePaymenInfoError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Parsing payment error: invalid payment info")
    }
}

impl fmt::Display for CreditCardError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Credit card error: Could not retrieve credit card.")
    }
}

impl Error for CreditCardError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {    
            CreditCardError::InvalidInput(_) => None,
            CreditCardError::Other(e, _) => {
                // as_ref will convert a type into a shared reference
                Some(e.as_ref())
            }
        }

    }
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
        CreditCardError::Other(Box::new(e), format!("{name}'s card could not be parsed."))
    })?;
    Ok(card)

}

fn parse_card(card: &str) -> Result<Card, ParsePaymenInfoError> {

    let mut numbers: Vec<u32> = parse_card_numbers(card)?;

    let len = numbers.len();
    let expected_len = 4;

    if len != expected_len {
        return Err(ParsePaymenInfoError{
            source: None,
            msg: format!("Incorrect number of elements parsed. Expected {expected_len} but got {len}. Elements: {numbers:?}")
        })
    }

    let cvv: u32 = numbers.pop().unwrap();
    let year: u32 = numbers.pop().unwrap();
    let month: u32 = numbers.pop().unwrap();
    let number: u32 = numbers.pop().unwrap();

    Ok( Card { number, exp: Expiration { month, year },  cvv } )

}

fn parse_card_numbers(card: &str) -> Result<Vec<u32>, ParsePaymenInfoError> {

    let numbers = card
        .split(" ")
        .into_iter()
        // Parse card numbers would now compile. Because we've implemented the From trait, 
        // the question mark operator automatically converts ParseIntError into 
        // ParsePaymentInfoError and propagates it.
        .map(|s| {
            s.parse()
            .map_err(|_| ParsePaymenInfoError {
                source: None,
                msg: format!("{s:?} could not be parsed as u32")
            })
        })
        .collect::<Result<Vec<u32>, _>>()
        .map_err(|e| ParsePaymenInfoError {
            source: Some(Box::new(e)),
            msg: format!("Failed to parse input as numbers. Input: {card}")
        })?;
    Ok(numbers)

}