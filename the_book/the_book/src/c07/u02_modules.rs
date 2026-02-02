//! # Defining Modules to Control Scope and Privacy

mod restaurant {
	mod hosting {
		fn add_to_waitlist() {}
		fn seat_at_table() {}
	}
	mod serving {
		fn take_order() {}
		fn serve_order() {}
		fn take_payment() {}
	}
}

fn modules_example() {
	let n1 = r#"
	pod: Crate Tree Example
	- `backyard`
	- `├── Cargo.lock`
	- `├── Cargo.toml`
	- `└── src`
	- `    ├── garden`
	- `    │   └── vegetables.rs`
	- `    ├── garden.rs`
	- `    └── main.rs`
	---"#;
	println!("{n1}");
}

fn modules_code() {
	let n1 = r#"
	pod: Module
	- Code within a module is private by default
	- Private items are internal implementation details
	- Group related definitions together
	- Sibling modules are defined in the same module
	---
	pod: Module A contained inside B
	- A is the `child` of module B
	- B is the `parent` of module A
	---
	pod: Module crate
	- Files `src/main.rs` and `src/lib.rs` are crate roots
	- Their content form a module name 'crate' at the root of module tree
	---
	pod: Module Tree
	- The crate's module structure
	---
	cmd:
	- `cargo new restaurant --lib`
	---"#;
	println!("{n1}");
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn run_modules_example() {
		modules_example();
	}

	#[test]
	fn run_modules_code() {
		modules_code();
	}
}
