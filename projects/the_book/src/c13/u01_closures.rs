use std::thread;

#[derive(Clone, Copy, Debug, PartialEq)]
enum ShirtColor {
	Blue,
	Red,
}

struct Inventory {
	shirts: Vec<ShirtColor>
}

impl Inventory {

	fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
		user_preference.unwrap_or_else(|| self.most_stocked())
	}

	fn most_stocked(&self) -> ShirtColor {
		let mut num_blue = 0;
		let mut num_red = 0;

		for color in &self.shirts {
			match color {
				ShirtColor::Blue => num_blue += 1,
				ShirtColor::Red => num_red += 1
			}
		}

		if num_blue > num_red {
			ShirtColor::Blue
		} else {
			ShirtColor::Red
		}

	}

}

fn closures_inventory() {
	let n1 = r#"
	pod: Closures
	- Anonymous functions you can save in a variable or pass as argument
	- Unlike functions, closures can capture values from the scope in which they're defined
	- If the closure has parameters, they appear between the two vertical pipes
	- The closure captures an immutable reference to the 'self' inventory instance and passes it with the code
	- A variable can bind to a closure definition, and we can latter use the variable and parentheses
	---"#;
	println!("{n1}");

	let store = Inventory {
		shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
	};

	let up1 = Some(ShirtColor::Red);
	let g1 = store.giveaway(up1);
	println!("Closures: User with preference {:?} gets {:?}", up1, g1);

	let up2 = None;
	let g2 = store.giveaway(up2);
	println!("Closures: User with preference {:?} gets {:?}", up2, g2);

}

fn closures_explicit() {

	let n1 = r#"
	pod: Closure Types
	- Type annotations are required in functions because the types are part of an explicit interface exposed to users
	- Closures aren't used in an exposed interface. They're stored in variables, and used without naming them
	- Closures are typically short and relevant only within a narrow context
	- The first time we call a closure the compiler infers its types. Those types are then locked into the closure
	---"#;
	println!("{n1}");

	// Function
	fn add_one_v1(x: u32) -> u32 { x + 1 };
	// Closures
	let add_one_v2 = |x: u32| -> u32 { x + 1 };
	let add_one_v3 = |x| { x + 1 };
	let add_one_v4 = |x| x + 1;

	let r2 = add_one_v2(10);
	let r3 = add_one_v3(10);
	let r4 = add_one_v4(10);
	println!("Closures: v2: {r2}, v3: {r3}, v4: {r4}");

	let same_value = |x| x;
	let s = same_value(String::from("hello"));

	println!("Closures: same value: {s}");

}

fn closures_references_immutable() {
	let n1 = r#"
	pod: Closures, references and ownership
	- Closure capture values: borrowing immutably, borrowing mutably and taking ownership.
	---"#;
	println!("{n1}");

	let v1 = vec![1, 2, 3];

	println!("Closures: before definition: v1: {v1:?}");
	let only_borrows = || println!("Closures: from closure: v1: {v1:?}");

	println!("Closures: before calling: v1: {v1:?}");
	only_borrows();

	println!("Closures: after calling: v1: {v1:?}");

}

fn closures_references_mutable() {
	let mut v2 = vec![5, 6, 7];

	println!("Closures: mutable before definition v2: {v2:?}");
	let mut borrows_mutably = || v2.push(8);

	borrows_mutably();
	println!("Closures: mutable after calling v2: {v2:?}");

}

fn closures_move_ownership() {

	let n1 = r#"
	pod: Closures moving ownership
	- Use the 'move' keyword before the parameter list
	- This technique is mostly used when passing a closure to a new thread to move the data
	---"#;
	println!("{n1}");

	let v3 = vec![5, 10, 15, 20];
	println!("Closures: before defining closure: v3: {v3:?}");

	thread::spawn(move || println!("Closures: from thread: v3: {v3:?}"))
		.join()
		.unwrap();

}

fn closures_values_out() {
	todo!("WIP")
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn run_closures_inventory() {
		closures_inventory();
	}

	#[test]
	fn run_closures_explicit() {
		closures_explicit();
	}

	#[test]
	fn run_closures_references_immutable() {
		closures_references_immutable();
	}

	#[test]
	fn run_closures_references_mutable() {
		closures_references_mutable();
	}

	#[test]
	fn run_closures_move_ownership() {
		closures_move_ownership();
	}

	#[test]
	fn run_closures_values_out() {
		closures_values_out()
	}
}
