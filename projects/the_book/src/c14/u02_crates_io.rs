//! # Publishing a Crate to Crates.io

fn crates_io() {
	let n1 = r#"
	pod: Documentation Comments
	- The HTML displays the documentation comments for public API items
	- Use three slashes and support Markdown notation
	- Sections that crate authors commonly use: Panics, Errors and Safety
	- Running 'cargo test' runs the code examples as tests
	---"#;
	println!("{n1}");
}

fn crates_documentation() {
	let n1 = r#"
	pod: Contained Item Comments
	- Using comment //! we're documenting the item that container this comment. In this case module 'u02_crates_io'
	---"#;
	println!("{n1}");
}

/// Adds one to the number given.
///
/// Examples
///
/// ```
/// let arg: i32 = 10;
/// let answer: i32 = the_book::c14::u02_crates_io::add_one(arg);
///
/// assert_eq!(11, answer);
/// ```
pub fn add_one(x: i32) -> i32 {
	x + 1
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn run_crates_io() {
		crates_io();
	}
}
