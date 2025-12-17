/**
 * pod: Panic
 * - Error state that is unrecoverable
 * - The program shows an error message and exits abruptly
 * - We can call the panic! macro
 * - For more verbose error trace we can set:
 *   - RUST_BACKTRACE=1
 *   - RUST_BACKTRACE=full
 */

fn unrecoverable_errors_panic_macro() {
	panic!("Something went wrong!");
}

fn unrecoverable_errors_bug() {
	let vec: Vec<&str> = vec!["one", "two", "three"];
	println!("{}", vec[3]);
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	#[should_panic]
	fn run_unrecoverable_errors_panic_macro() {
		println!("panic macro");
		unrecoverable_errors_panic_macro();
	}

	#[test]
	#[should_panic]
	fn run_unrecoverable_errors_bug() {
		println!("vector index");
		unrecoverable_errors_bug();
	}
}
