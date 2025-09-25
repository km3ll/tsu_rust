enum Command {
	Undo,
	Redo,
	AddText(String),
	MoveCursor(i32, i32),
	Replace { from: String, to: String },
}

impl Command {
	fn serialize(&self) -> String {
		match self {
			Command::Undo => String::from("{ \"cmd\": \"undo\" }"),
			Command::Redo => String::from("{ \"cmd\": \"redo\" }"),
			Command::AddText(text) => {
				format!(
					"{{ \
                        \"cmd\": \"add_text\", \
                        \"text\": \"{text}\" \
                    }}"
				)
			}
			Command::MoveCursor(line, column) => {
				format!(
					"{{ \
                        \"cmd\": \"move_cursor\", \
                        \"line\": \"{line}\", \
                        \"column\": \"{column}\"\
                    }}"
				)
			}
			Command::Replace { from, to } => {
				format!(
					"{{ \
                        \"cmd\": \"replace\", \
                        \"from\": \"{from}\", \
                        \"to\": \"{to}\" \
                    }}"
				)
			}
		}
	}
}

pub fn matching_expression() {
	/**
	 * pod: Match Expression
	 * - Flow-control operator
	 * - Allows to compare a value against the series of patterns (match arms)
	 *   to determine which code path to execute
	 * - Exhaustive on match arms
	 * - Patterns can be literal values, ranges, variable names, wildcards, etc
	 */
	let age: u32 = 32;
	match age {
		// Literal value
		1 => println!("Happy 1st Birthday!"),
		// Range
		13..19 => println!("You're a teenager!"),
		// With binding value
		x => println!("You are {x} years old!"), // Catch-all pattern (no binding value)
		// _ => println!("Other age")
	}
}

pub fn matching_serialize() {
	let cmd1 = Command::Undo;
	let cmd2 = Command::Redo;
	let cmd3 = Command::AddText(String::from("Ferris"));
	let cmd4 = Command::Replace {
		from: String::from("Hello"),
		to: String::from("Hola"),
	};
	println!("cmd1: {}", cmd1.serialize());
	println!("cmd2: {}", cmd2.serialize());
	println!("cmd3: {}", cmd3.serialize());
	println!("cmd4: {}", cmd4.serialize());
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn run_matching_expression() {
		matching_expression();
	}

	#[test]
	fn run_matching_serialize() {
		matching_serialize();
	}
}
