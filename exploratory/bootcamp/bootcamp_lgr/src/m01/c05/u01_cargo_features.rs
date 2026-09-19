//! # Cargo Features

pub fn features() {
    let n1 = r#"
	pod: Cargo Features
	- Allow to define part of code that are conditionally compiled, only if a certain feature is enabled
	- Allow to define optional dependencies
	- Reduce compile time and file sizes
	- Disabled by default
	---
	pod: Cargo.toml
	- Features have a name and an associated array: other features or optional dependencies
	- Dependencies must be marked as optional
	---"#;
    println!("{n1}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_features() {
        features();
    }
}
