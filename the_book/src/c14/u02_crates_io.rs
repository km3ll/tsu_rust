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

fn crates_re_exporting() {
	let n1 = r#"
	pod: Exporting Public APIs
	- You can re-export items to make a public structure that's different from your native structure
	- A common use of 'pub use' is to re-export definitions of a dependency in the current crate
	---"#;
	println!("{n1}");
}

fn crates_publishing() {
	let n1 = r#"
	pod: Publishing a Crate
	- Run the 'cargo login' command, which uses your API token and stores it locally in '.cargo/credentials.toml'
	- Add metadata in the [package] section of Cargo.toml
	- A publish are permanent
	- To publish a new version, change the version value in your Cargo.toml file and republish
	---"#;
	println!("{n1}");
}

fn crates_yanking() {
	let n1 = r#"
	pod: Yanking a Version
	- Prevents any future projects from adding a version as new dependency
	- All projects with a Cargo.lock will not break, and any future Cargo.lock files will not use the yanked version
	- A yank doesn't delete any code (such as accidentally uploaded secrets)
	- You can also undo a yank and allow projects to start depending on a version again
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

	#[test]
	fn run_crates_re_exporting() {
		crates_re_exporting()
	}

	#[test]
	fn run_crates_publishing() {
		crates_publishing()
	}

	#[test]
	fn run_crates_yanking() {
		crates_yanking()
	}
}
