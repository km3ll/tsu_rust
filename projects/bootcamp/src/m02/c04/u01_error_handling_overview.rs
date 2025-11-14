use std::error::Error;
use std::fmt::{Debug, Display};

/**
 * pod: Error Handling
 * - defining
 * - propagating
 * - handling or discarding
 * - reporting (end-users & developers)
 */

/**
 * pod: Audiences
 * - Machines
 * - End-Users
 * - Developers
 */

/**
 * pod: Types
 * - recoverable
 * - non-recoverable
 */

/**
 * pod: Error trait
 * - Minimum requirement for an error
 * - source: underlying cause of the error
 * - return type: a reference to a trait object, wich
 *   could be any type that implements the Error and static traits
 * - static trait bound: it cannot have any non-static references.
 *   We can hold on to the type as long as we want because the
 *   memory will always be valid
 * - two super traits: Debug (developers) and Display (users)
 */
fn error_overview_error_trait() {
	/**
	 * pod: Idiomatic errors
	 * - semantically mark types as errors
	 * - standardizes
	 *   - checking the source
	 *   - user-facing reporting
	 *   - developer-facing reporting
	 */
	trait Error: Debug + Display {
		fn source(&self) -> Option<&(dyn Error + 'static)> {
			None
		}
	}
	println!("idiomatic errors");
}

/**
 * pod: Custom errors (dynamic)
 */
struct ServerError {
	code: u8,
	body: String,
	source: Box<dyn Error>,
}

/**
 * pod: Custom errors (enum)
 * - Useful when you want to change your code's behavior
 *   based on the different variants
 */
enum APIError {
	UserInputError(String),
	InternalError(Box<dyn Error>),
}

/**
 * pod: Custom errors (mix)
 */
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
	fn run_error_overview_error_trait() {
		error_overview_error_trait();
	}

	#[test]
	fn run_error_overview_custom() {
		error_overview_custom();
	}
}
