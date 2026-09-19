//! # Structs & Lifetime Elision

#[derive(Debug)]
struct TweetV1 {
    content: String,
}

#[derive(Debug)]
struct TweetV2<'a> {
    content: &'a str,
}

impl<'a> TweetV2<'a> {
    fn replace_content(&mut self, content: &'a str) -> &str {
        let old_content = self.content;
        self.content = content; // error: explicit lifetime required in the type of `content`
        old_content
    }
}

fn lifetime_elision() {
    let n1 = r#"
    pod: config: allow
    - Locally: `#[allow(mismatched_lifetime_syntaxes)]`
    - Globally: at crate level `#![allow(mismatched_lifetime_syntaxes)]`
    ---
    pod: References in Structs
    - We must add generic lifetime annotations
    ---
    pod: Implementation Block
    - We must include generic lifetime annotations
    ---
    pod: 3rd Elision Rule
    - If there are multiple input lifetime parameters, but one of them is `&self` or `&mut self`, the lifetime of self is assigned to all output lifetime parameters
    ---
    pod: Lifetime Elision Rules
    - Each parameter that is a reference gets its own lifetime parameter
    - If there is exactly one input lifetime parameter, that lifetime is assigned to all output lifetime parameters
    - If there are multiple input lifetime parameters, but one of them is `&self` or `&mut self`, the lifetime of self is assigned to all output lifetime parameters
    ---
    pod: Lifetimes
    - Input: lifetime of function parameters
    - Output: lifetime of return values
    ---"#;
    println!("{n1}");
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
    fn run_lifetime_elision() {
        lifetime_elision()
    }

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
