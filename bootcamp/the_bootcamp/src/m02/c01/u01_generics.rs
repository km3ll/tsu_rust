/**
 * pod: Generics conventions
 * - Start in T (uppercase), which represents Type
 * - Use T + Integer, as in T0, T1, etc.
 * - Use Camel Case as in PayloadType
 */
#[derive(Debug)]
struct Command<T> {
	name: String,
	payload: T,
}

/**
 * pod: Implementation block for any Command type
 */
impl<T> Command<T> {
	/**
	 * pod: Self return type
	 * - If we change the name of the struct, we don't have
	 *   to change the return type.
	 */
	fn new(name: String, payload: T) -> Self {
		Command {
			name: name,
			payload: payload,
		}
	}

	/**
	 * pod: Generics in method
	 */
	fn get_payload(&self) -> &T {
		&self.payload
	}
}

/**
 * pod: Implementation block for a concrete type
 */
impl Command<String> {
	fn print_payload(&self) {
		println!("payload: {}", self.payload)
	}
}

pub fn generics_structs() {
	let cmd1: Command<String> = Command {
		name: "navigate".to_owned(),
		payload: "www.google.com".to_owned(),
	};

	let cmd2: Command<i32> = Command {
		name: "zoom".to_owned(),
		payload: 200,
	};
	println!("cmd1: {:?}", cmd1);
	println!("cmd2: {:?}", cmd2);
}

pub fn generics_impl_block_generic() {
	let cmd3: Command<String> = Command::new("navigate".to_owned(), "www.google.com".to_owned());

	let cmd4: Command<i32> = Command::new("zoom".to_owned(), 200);

	println!("cmd3: {:?}", cmd3);
	println!("cmd4: {:?}", cmd4);
}

pub fn generics_impl_block_concrete() {
	let cmd5: Command<String> = Command::new("navigate".to_owned(), "www.google.com".to_owned());
	cmd5.print_payload();
}

pub fn generics_method() {
	let cmd5: Command<String> = Command {
		name: "navigate".to_owned(),
		payload: "www.google.com".to_owned(),
	};

	let cmd6: Command<i32> = Command {
		name: "zoom".to_owned(),
		payload: 200,
	};

	let pl5: &String = cmd5.get_payload();
	let pl6: &i32 = cmd6.get_payload();

	println!("pl5: {}", pl5);
	println!("pl6: {}", pl6);
}

pub fn generics_emum() {
	enum Option<T> {
		Some(T),
		None,
	}
	let e1 = Some("Hello".to_owned());
	println!("e1: {:?}", e1)
}

/**
 * pod: Monomorphization
 * - Rust resolves generics at compile time as concrete implementations
 */
fn serialize_payload<T>(payload: T) -> String {
	// Convert into JSON string
	"placeholder".to_owned()
}

pub fn generics_free_function() {
	let cmd7: Command<i32> = Command {
		name: "zoom".to_owned(),
		payload: 200,
	};
	let pl7: &i32 = cmd7.get_payload();
	serialize_payload(pl7);
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn run_generics_structs() {
		generics_structs();
	}

	#[test]
	fn run_generics_impl_block_generic() {
		generics_impl_block_generic();
	}

	#[test]
	fn run_generics_impl_block_concrete() {
		generics_impl_block_concrete();
	}

	#[test]
	fn run_generics_method() {
		generics_method();
	}

	#[test]
	fn run_generics_enum() {
		generics_emum();
	}

	#[test]
	fn run_generics_free_function() {
		generics_free_function();
	}
}
