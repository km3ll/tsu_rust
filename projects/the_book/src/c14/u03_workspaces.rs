//! # Cargo Workspaces

fn workspaces() {
	let n1 = r#"
	pod: Workspaces
	- To split your package further into multiple library crates
	- A set of packages that share the same Cargo.lock and output directory
	- Crates in a workspace are meant to depend on each other
	---"#;
	println!("{n1}");
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn run_workspaces() {
		workspaces();
	}
}
