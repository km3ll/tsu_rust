fn run(query: String) -> Result<String, String> {
	// pod: Parenthesis can be omitted
	if query.is_empty() {
		Err(String::from("Query is empty"))
	} else {
		Ok(String::from("Ferris"))
	}
}

fn find_name(query: String) -> Option<String> {
	// pod: Convert Result into Option
	let result: Result<String, String> = run(query);
	result.ok()
}

pub fn result_enum() {
	/**
	 * pod: Result is defined and loaded in the prelude
	 */
	let rs1 = run(String::from(""));
	println!("rs1: {:?}", rs1);
	let rs2 = run(String::from("query"));
	println!("rs2: {:#?}", rs2);
}

pub fn result_to_option() {
	let op1 = find_name(String::from(""));
	println!("op1: {:#?}", op1);
	let op2 = find_name(String::from("query"));
	println!("op2: {:#?}", op2);
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn run_result_enum() {
		result_enum();
	}

	#[test]
	fn run_result_to_option() {
		result_to_option();
	}
}
