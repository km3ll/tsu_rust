//! # Running Code on Cleanup with the Drop Trait

#[derive(Debug)]
struct MyPointer {
	data: String,
}

impl Drop for MyPointer {
	fn drop(&mut self) {
		println!("Drop: {:?}", self);
	}
}

fn drop_trait() {
	let n1 = r#"
	pod: trait: Drop
	- To customize what happens when a value is about to go out of scope
	- Is almost always used when implementing smart pointers
	- The compiler inserts Drop code automatically on any value
	---"#;
	println!("{n1}");

	let mp1 = MyPointer {
		data: String::from("Hello"),
	};
	{
		let mp2 = MyPointer {
			data: String::from("Ferris!"),
		};
	}
}

fn drop_destructor() {
	let n1 = r#"
	pod: Destructor
	- Programming term for a function that cleans up an instance
	- Rust doesn't let us call drop, because it will call drop at the end of main (double free error)
	- To force-drop we can call function std::mem:drop
	---"#;
	println!("{n1}");

	let mp1 = MyPointer {
		data: String::from("Hello"),
	};
	drop(mp1);
	let mp2 = MyPointer {
		data: String::from("Ferris!"),
	};
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn run_drop_trait() {
		drop_trait();
	}

	#[test]
	fn run_drop_destructor() {
		drop_destructor()
	}
}
