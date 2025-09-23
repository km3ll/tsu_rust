#![allow(unused)]
use std::error::Error;

fn main() {

    /**
     * Errors have three audiences:
     * 1. Machines, which need error for control flow
     * 2. End users
     * 3. Developer, which need detailed diagnostics to debug
     * 
     * There are two types of errors
     * - Nonrecoverable, which can be thrown with the 'panic' macro
     * - Recoverable
     */
    /// 3. 

    println!("Hello, world!");

}

/**
 * Idiomatic erros must implement the Error trait.
 * 
 * The static trait bound means that the type cannot have any non-static
 * references. This means we can hold to the type as long as we want because
 * because the memory will always be valid
 * 
 * The Error trait has two super traits: Debug and Display 
 * 
 * pub trait Error: Debug + Display {
 *    fn source(&self) -> Option<&(dyn Error + 'static)> {
 *        None
 *    }
 * }
 * 
 * Custom error types can be structs or enums
 * 
 */

struct ServerError {
    status_code: u8,
    body: String,
    source: Box<dyn Error> // The underlying error
}

enum APIError {
    UserInputError(String),
    InternalError(Box<dyn Error>) // The underlying error
}