pub fn workspace_virtual_manifest() {
	/**
	 * pod: Workspace
	 * - A collection of related cargo packages.
	 * - Packages in a workspace share a Cargo.lock file, meaning that if multiple
	 *   packages have the same dependency, it will be resolved to one version.
	 * - Packages in a workspace are called 'members' and they share one output directory.
	 */
	println!("workspace");
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn run_workspace_virtual_manifest() {
		workspace_virtual_manifest();
		assert!(true)
	}
}
