/**
 * pod: allow
 * - locally: #[allow(mismatched_lifetime_syntaxes)]
 * - globally at crate level: #![allow(mismatched_lifetime_syntaxes)]
 */

#[derive(Debug)]
struct TweetV1 {
	content: String,
}

/**
 * pod: References in Structs
 * - We must add a generic lifetime annotations
 */
#[derive(Debug)]
struct TweetV2<'a> {
	content: &'a str,
}

/**
 * pod: Implementation block
 * - We must include generic lifetime annotations
 *
 * pod: 3rd elision rule applied
 * - If there are multiple input lifetime parameters, but one of them is
 *   &self or &mut self, the lifetime of self is assigned to all output
 *   lifetime parameters
 */
impl<'a> TweetV2<'a> {
	fn replace_content(&mut self, content: &'a str) -> &str {
		let old_content = self.content;
		self.content = content; // error: explicit lifetime required in the type of `content`
		old_content
	}
}

fn lifetime_elision_owned() {
	let t1 = TweetV1 {
		content: "Hello Ferris".to_owned(),
	};
	println!("t1: {:?}", t1)
}

fn lifetime_elision_reference() {
	let t2 = TweetV2 {
		content: "Hello Ferris",
	};
	println!("t2: {:?}", t2)
}

fn lifetime_elision_replaced() {
	let mut t3: TweetV2<'_> = TweetV2 { content: "Ferris" };

	println!("t3: {:?}", t3);
	let old = t3.replace_content("Pod!");
	println!("t3: {:?}", t3)
}

/**
 * pod: Lifetime elision rules
 * - Each parameter that is a reference gets its own lifetime parameter
 * - If there is exactly one input lifetime parameter, that lifetime
 *   is assigned to all output lifetime parameters.
 * - If there are multiple input lifetime parameters, but one of them is
 *   &self or &mut self, the lifetime of self is assigned to all output
 *   lifetime parameters
 */

/**
 * pod: Lifetimes
 * - input: lifetime of function parameters
 * - output: lifetime of return values
 */
fn take_and_return_content_rule_1<'a>(content: &'a str) -> &str {
	content
}

fn take_and_return_content_rule_2<'a>(content: &'a str) -> &'a str {
	content
}

// -> &str missing lifetime specifier
fn take_and_return_content_rule_3<'a, 'b>(content1: &'a str, content2: &'b str) -> &'a str {
	content1
}

fn lifetime_elision_rules() {
	let c1 = take_and_return_content_rule_1("Hello");
	let c2 = take_and_return_content_rule_2("Hello");
	let c3 = take_and_return_content_rule_3("Hello", "Pod");
	println!("c1: {}", c1);
	println!("c2: {}", c2);
	println!("c3: {}", c3);
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn run_lifetime_elision_owned() {
		lifetime_elision_owned();
	}

	#[test]
	fn run_lifetime_elision_reference() {
		lifetime_elision_reference();
	}

	#[test]
	fn run_lifetime_elision_replaced() {
		lifetime_elision_replaced();
	}

	#[test]
	fn run_lifetime_elision_rules() {
		lifetime_elision_rules();
	}
}
