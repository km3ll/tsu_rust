//! # Overview of Error Handling

use std::error::Error;
use std::fmt::{Debug, Display};

fn error_handling() {
    let n1 = r#"
    pod: Error Handling
    - Defining, propagating, handling, discarding and reporting
    ---
    pod: Error Audiences
    - End-users, machines and/or developers
    ---
    pod: Error Minimum Requirements
    - Source: underlying cause of the error
    - Return Type: a reference to a trait object, which could be any type that implements the Error and static traits
    - Static trait bound: it cannot have any non-static references. We can hold on to the type as long as we want because the memory will always be valid
    ---
    pod: Error Super Traits
    - `Debug` for developers and `Display` for users
    ---
    pod: Idiomatic Errors
    - Semantically mark types as errors
    - Standardizes: (1) checking the source, (2) user-facing reporting and (3) developer-facing reporting
    ---
    pod: Custom Errors
    - Use dynamic types: `Box<dyn Error>`
    - Useful when you want to change your code's behavior based on the different variants
    ---"#;
    println!("{n1}");
}

fn error_overview_error_trait() {
    trait Error: Debug + Display {
        fn source(&self) -> Option<&(dyn Error + 'static)> {
            None
        }
    }
    println!("idiomatic errors");
}

struct ServerError {
    code: u8,
    body: String,
    source: Box<dyn Error>,
}

enum APIError {
    UserInputError(String),
    InternalError(Box<dyn Error>),
}

enum APIErrorType {
    UserInputError,
    InternalError,
}

struct APIErrorV2 {
    msg: String,
    source: Option<Box<dyn Error>>,
    err_type: APIErrorType,
}

fn error_overview_custom() {
    println!("custom errors");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_error_handling() {
        error_handling()
    }

    #[test]
    fn run_error_overview_error_trait() {
        error_overview_error_trait();
    }

    #[test]
    fn run_error_overview_custom() {
        error_overview_custom();
    }
}
