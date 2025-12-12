use std::net::IpAddr;

fn game_v1() {
	loop {
		let guess = "30";
		let guess: i32 = match guess.trim().parse() {
			Ok(num) => num,
			Err(_) => continue,
		};

		if guess < 1 || guess > 100 {
			println!("The secret number must be between 1 and 100.");
			continue;
		}
	}
}

struct Guess {
	value: i32,
}

impl Guess {
	pub fn new(value: i32) -> Guess {
		if value < 1 || value > 100 {
			panic!("Guess value must be between 1 and 100, got {value}");
		}
		Guess { value }
	}

	pub fn value(&self) -> i32 {
		self.value
	}
}

fn panic_not_basic() {
	let n1 = r#"
	pod: To panic! or Not To panic!
	- When code panics, there's no way to recover
	- Returning 'Result' is a good default choice in a function that might fail
	- In situations such as examples, prototypes and tests, it's more appropriate to write code that panics
	- The 'unwrap' and 'expect' methods are very handy when prototyping, before you're ready to decide how to handle errors
	- It's appropriate to use 'expect' when you have some logic that the compiler doesn't understand
	- It's advisable to panic when it's possible that your code could end up in a bad state
	- It's advisable to panic in cases where continuing could be insecure or harmful
	- It's more appropriate to return a Result when failure is expected
	---"#;
	println!("{n1}");

	let home: IpAddr = "127.0.0.1"
		.parse()
		.expect("Hardcoded IP address should be valid");
	println!("Panic: home: {home}");
}

fn panic_not_contracts() {
	let n1 = r#"
	pod: Contracts
	- Functions often have contracts: their behavior is only guaranteed if the inputs meet particular requirements
	- Panicking when the contract is violated makes sense
	---"#;
	println!("{n1}");
}

fn panic_construction() {
	let n1 = r#"
	pod: Controller Construction
	- We can make a new type and put the validations in a function to create an instance rather than repeating the validations everywhere
	---"#;
	println!("{n1}");
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn run_panic_not_basic() {
		panic_not_basic();
	}

	#[test]
	fn run_panic_not_contracts() {
		panic_not_contracts();
	}

	#[test]
	fn run_panic_construction() {
		panic_construction();
	}
}
