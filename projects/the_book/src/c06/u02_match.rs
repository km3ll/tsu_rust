#[derive(Debug)]
enum CoinV1 {
	Penny,
	Nickel,
	Dime,
	Quarter,
}

#[derive(Debug)]
enum CoinV2 {
	Penny,
	Nickel,
	Dime,
	Quarter(UsState),
}

#[derive(Debug)]
enum UsState {
	Alabama,
	Alaska,
	Arizona,
}

fn value_in_cents_v1(coin: CoinV1) -> u8 {
	match coin {
		CoinV1::Penny => {
			println!("Match: lucky penny: {:?}", coin);
			1
		}
		CoinV1::Nickel => 5,
		CoinV1::Dime => 10,
		CoinV1::Quarter => 25,
	}
}

fn value_in_cents_v2(coin: CoinV2) -> u8 {
	match coin {
		CoinV2::Penny => {
			println!("Match: lucky penny: {:?}", coin);
			1
		}
		CoinV2::Nickel => 5,
		CoinV2::Dime => 10,
		CoinV2::Quarter(state) => {
			println!("Match: Quarter from state: {state:?}");
			25
		},
	}
}

fn match_definition() {
	let n1 = r#"
	pod: Match
	- Compares a value against a series of patterns
	- The compiler confirms that all possible cases are handled
	- Match arms have two parts: a pattern and some code
	- The code associated with each arm is an expression
	---"#;
	println!("{n1}");

	let coin = CoinV1::Penny;
	value_in_cents_v1(coin);
}

fn match_binding() {
	let n1 = r#"
	pod: Match Arms
	- They can bind to the parts of the values that match the pattern
	---"#;
	println!("{n1}");

	let coin = CoinV2::Quarter(UsState::Alabama);
	value_in_cents_v2(coin);
}

fn match_() {

}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn run_match_definition() {
		match_definition();
	}

	#[test]
	fn run_match_binding() {
		match_binding();
	}

	#[test]
	fn run_() {
		match_();
	}
}
