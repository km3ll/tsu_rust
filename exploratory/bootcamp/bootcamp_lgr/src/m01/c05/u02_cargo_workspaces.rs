//! # Cargo Workspaces

pub fn workspaces() {
    let n1 = r#"
	pod: Workspace
	- A collection of related cargo packages
	- Packages in a workspace share a `Cargo.lock` file
	- If multiple packages have the same dependency, it will be resolved to one version
	- Packages in a workspace are called `members` and they share one output directory
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
