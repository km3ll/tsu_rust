#![allow(unused)]

fn main() {

    // Constants cannot be mutated with the keyword 'mut'
    // Naming convention: screaming snake case
    // Their value must be computed at compile time
    const MAX_PLAYERS: u8 = 10;
    static CASINO_NAME: &str = "Rusty Casino";

    // When using constant variables their value will be inline
    // Constants do not ocupy a specific location in memory
    let a: u8 = MAX_PLAYERS;
    let b: u8 = MAX_PLAYERS;

    // Statics do occupy a specific location in memory, which means
    // there is only one instance of the value

    let c: &str = CASINO_NAME;
    let d: &str = CASINO_NAME;

}