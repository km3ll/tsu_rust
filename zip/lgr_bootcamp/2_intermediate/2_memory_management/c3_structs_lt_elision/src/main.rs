#![allow(unused)]
// When storing references in structs we must add a generic lifetime annotation

struct Tweet<'a> {
    content: &'a str,
}

// We must include generic lifetime annotations inside the definition of impl blocks
impl<'a> Tweet<'a> {

    fn replace_content(&mut self, content: &'a str) -> &str {
        let old_content = self.content;
        // Error: explicit lifetime required in the type of content
        self.content = content;
        old_content
    }

}

fn main() {

    let mut tweet: Tweet = Tweet {
        content: "example"
    };

    let old_content: &str = tweet.replace_content("replace example");
    println!("old content: {}", old_content);
    println!("new content: {}", tweet.content);
    
}

// Lifetime Elision Rules
// 1. Each parameter that is a reference gets its own lifetime parameter.
// 2. If there is exactly one input lifetime parameter, that lifetime
//    is assigned to all output lifetime parameters.
// 3. If there are multiple input parameters, but one of them is 
//    &self or &mut self, the lifetime of self is assigned to all output
//    lifetime parameters.

//    Input lifetimes  = parameters
//    Output lifetimes = return values

fn take_and_return_content_rules_1_and_2(content: &str) -> &str {
    content
}

fn take_and_return_content_rules_1<'a>(content1: &'a str, content2: &'a str) -> &'a str {
    content1
}