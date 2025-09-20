use rand::prelude::*;

pub fn dependencies_crates_io() {
	let timeout = rand::rng().random_range(100..500);
	println!("timeout: {}", timeout);
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn run_dependencies_crates_io() {
		dependencies_crates_io();
	}
}
