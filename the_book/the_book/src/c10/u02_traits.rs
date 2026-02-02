//! # Defining Shared Behavior with Traits

use serde::Deserialize;
use std::fmt::{Debug, Display};
use std::thread::sleep;

pub trait Summary {
	fn summarize_author(&self) -> String;

	fn summarize(&self) -> String {
		format!("(Read more from {}...)", self.summarize_author())
	}
}

#[derive(Debug)]
pub struct NewsArticle {
	pub headline: String,
	pub location: String,
	pub author: String,
	pub content: String,
}

impl Summary for NewsArticle {
	fn summarize_author(&self) -> String {
		format!("@{}", self.author)
	}
	// Default implementation
}

#[derive(Debug)]
pub struct SocialPost {
	pub username: String,
	pub content: String,
	pub reply: bool,
	pub repost: bool,
}

impl Summary for SocialPost {
	fn summarize_author(&self) -> String {
		format!("@{}", self.username)
	}
	fn summarize(&self) -> String {
		format!("{}: {}", self.username, self.content)
	}
}

pub fn notify_v1(item: &impl Summary) {
	println!("Traits: as parameters v1: {}", item.summarize())
}

pub fn notify_v2<T: Summary>(item: &T) {
	println!("Traits: as bound syntax v2: {}", item.summarize());
}

pub fn notify_v3(item: &(impl Summary + Debug)) {
	println!("Traits: as multiple bounds v3: {}", item.summarize());
}

pub fn notify_v4<T: Summary + Debug>(item: &T) {
	println!("Traits: as multiple bounds v4: {}", item.summarize());
}

pub fn notify_v5<T, U>(item1: &T, item2: &U)
where
	T: Summary + Debug,
	U: Summary + Debug,
{
	println!("Traits: as bound where clauses v5: {}", item1.summarize());
}

fn returns_summarizable() -> impl Summary {
	SocialPost {
		username: String::from("Lorem Ipsum"),
		content: String::from("Dummy text of the printing and typesetting industry."),
		reply: false,
		repost: true,
	}
}

struct PairV2<T> {
	x: T,
	y: T,
}

impl<T> PairV2<T> {
	fn new(x: T, y: T) -> Self {
		Self { x, y }
	}
}

impl<T: Display + PartialOrd> PairV2<T> {
	fn cmd_display(&self) {
		println!("x: {}, y: {}", self.x, self.y);
	}
}

fn traits_definition() {
	let n1 = r#"
	pod: Trait
	- We can define shared behavior in an abstract way
	- Trait bounds specify that a generic type can be any type that has certain behavior
	- Behavior consists of the methods we can call on a type
	- To use a default implementation we specify an empty impl block
	- Default implementations can call other methods in the same trait, even if those don't have a default implementation
	- It isn't possible to call the default implementation from an overriding implementation of the same method
	---"#;
	println!("{n1}");

	let post = SocialPost {
		username: String::from("Lorem Ipsum"),
		content: String::from("Dummy text of the printing and typesetting industry."),
		reply: false,
		repost: false,
	};
	println!("Traits: 1 new post: {}", post.summarize());
}

fn traits_coherence() {
	let n1 = r#"
	pod: Coherence or The Orphan Rule
	- We can implement a trait on a type only if either the trait or the type, or both, are local to our crate
	---"#;
	println!("{n1}");
}

fn traits_default_implementation() {
	let article = NewsArticle {
		headline: String::from("Stanley Cup Championship"),
		location: String::from("Pittsburgh, PA, USA"),
		author: String::from("Iceburgh"),
		content: String::from("The Pittsburgh Penguins are the best hokey team in the NHL"),
	};
	println!(
		"Traits: default implementation of article: {}",
		article.summarize()
	);
}

fn traits_as_parameters() {
	let post = SocialPost {
		username: String::from("Lorem Ipsum"),
		content: String::from("Dummy text of the printing and typesetting industry."),
		reply: false,
		repost: true,
	};
	notify_v1(&post);
}

fn traits_bound_syntax() {
	let n1 = r#"
	pod: Trait Bound Syntax
	- Is equivalent to the 'impl Trait' syntax
	- A generic type constraints the function
	- Multiple bounds are expressed with the `+` syntax
	---"#;
	println!("{n1}");

	let post = SocialPost {
		username: String::from("Lorem Ipsum"),
		content: String::from("Dummy text of the printing and typesetting industry."),
		reply: false,
		repost: true,
	};
	notify_v2(&post);
}

fn traits_multiple_bounds() {
	let post = SocialPost {
		username: String::from("Lorem Ipsum"),
		content: String::from("Dummy text of the printing and typesetting industry."),
		reply: false,
		repost: true,
	};
	notify_v3(&post);
	notify_v4(&post);
}

fn traits_where_syntax() {
	let n1 = r#"
	pod: Trait 'where' Syntax
	- Each generic has its own trait bounds
	---"#;
	println!("{n1}");

	let post = SocialPost {
		username: String::from("Lorem Ipsum"),
		content: String::from("Dummy text of the printing and typesetting industry."),
		reply: false,
		repost: true,
	};
	notify_v5(&post, &post);
}

fn traits_returning_impl() {
	let n1 = r#"
	pod: Return 'impl Trait' syntax
	- Lets specify that a function returns some type that implements a trait
	- The syntax only works if you're returning a single type (either of two implementations doesn't work)
	---
	pod: Blanket Implementations
	- Conditionally implementing a trait for any type that implements another trait
	- The standard library implements ToString on any type that implements Display
	- We can call the to_string method on integers, for example
	- impl<T: Display> ToString for T {...}
  	- Blanket implementations appear in the documentation for the trait in the 'implementors' section
	---"#;
	println!("{n1}");

	let summarizable = returns_summarizable();
	println!("Traits: returning impl Trait: {}", summarizable.summarize());
}

fn traits_conditional_impl() {
	let n1 = r#"
	pod: Trait Bounds to Conditionally Implement Methods
	- Always implements new() on PairV2
	- Implements cmd_display() if Display and PartialOrd implemented
	---"#;
	println!("{n1}");
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn run_traits_definition() {
		traits_definition();
	}

	#[test]
	fn run_traits_coherence() {
		traits_coherence();
	}

	#[test]
	fn run_traits_default_implementation() {
		traits_default_implementation();
	}

	#[test]
	fn run_traits_as_parameters() {
		traits_as_parameters();
	}

	#[test]
	fn run_traits_bound_syntax() {
		traits_bound_syntax();
	}

	#[test]
	fn run_traits_multiple_bounds() {
		traits_multiple_bounds();
	}

	#[test]
	fn run_traits_where_syntax() {
		traits_where_syntax();
	}

	#[test]
	fn run_traits_returning_impl() {
		traits_returning_impl();
	}

	#[test]
	fn run_traits_conditional_impl() {
		traits_conditional_impl();
	}
}
