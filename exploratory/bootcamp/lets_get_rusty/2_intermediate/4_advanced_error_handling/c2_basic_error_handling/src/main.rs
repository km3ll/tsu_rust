#![allow(dead_code, unused)]

use std::{ collections::HashMap, io, num::ParseIntError };


fn main() {

    find_credit_card("Amy");
    find_credit_card("Tim");
    find_credit_card("Bob");
    find_credit_card("John");

}



fn find_credit_card(name: &str) {

    let credit_cards = HashMap::from([
        ("Amy", "1234567 04 25 123"),
        ("Tim", "1234567 06 27"),      // Invalid record
        ("Bob", "1234567 Dec 27 123"), // Invalid record
    ]);

    /*
    println!("Enter your name: ");
    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read line");
    */

    let result = get_credit_card_info(&credit_cards, name.trim());
        
    match result {
        Ok(card) => println!("\nCredit card info: {card:?}"),
        Err(e) => println!("\nError: {e}"),
    }

}

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

fn get_credit_card_info(
    credit_cards: &HashMap<&str, &str>,
    name: &str,
) -> Result<Card, String> {

    let card_string: &str = credit_cards.get(name)
        .ok_or(format!{"No credit card was found for {name}."})?;
    let card = parse_card(card_string)?;
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