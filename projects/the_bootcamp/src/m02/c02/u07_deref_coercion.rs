use std::ops::{Deref, DerefMut};

struct MySmartPointer<T> {
	value: T,
}

impl<T> MySmartPointer<T> {
	fn new(value: T) -> MySmartPointer<T> {
		MySmartPointer { value }
	}
}

impl<T> Deref for MySmartPointer<T> {
	type Target = T;
	fn deref(&self) -> &Self::Target {
		&self.value
	}
}

impl<T> DerefMut for MySmartPointer<T> {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.value
	}
}

/**
 * pod: Implicit Deref Coercion
 * - The type we're passing in is different from the type accepted
 * - To coerce a reference of one type into a reference of another type
 *   &Box -> &String -> &str
 * - Only works on types that implement the 'Deref' and 'Deref mut' traits
 * - DeRef = DeReference
 * - DeRef operator = '*'
 *   The compiler generates this sequence of deref calls implicitly
 * - Deref and Deref mut should only be implemented on smart pointer types
 */
fn print_ref(s: &str) {
	println!("s: {s}");
}

fn deref_coercion_base() {
	let s1: Box<String> = Box::new("Hello Ferris".to_owned());
	print_ref(&s1);
}

fn deref_coercion_my_smart_pointer() {
	// &MySmartPointer -> &Box -> &String -> &str
	let s1: MySmartPointer<Box<String>> = MySmartPointer::new(Box::new("Hello Ferris".to_owned()));
	print_ref(&s1);
}

fn deref_coercion_deref_operator_immutable() {
	let s1: MySmartPointer<Box<String>> = MySmartPointer::new(Box::new("Hello Ferris".to_owned()));
	print_ref(&s1);

	let s2: &MySmartPointer<Box<String>> = &(s1);
	print_ref(s2);

	// One Dereference operator (*)
	let s3: &Box<String> = &*(s1);
	print_ref(&s3);

	// Two Dereference operators (**)
	let s4: &String = &(**s1);
	print_ref(s4);

	let s5: &str = &(***s1);
	print_ref(s5);
}

fn deref_coercion_deref_operator_mutable() {
	let mut s1: MySmartPointer<Box<String>> =
		MySmartPointer::new(Box::new("Hello Ferris".to_owned()));
	print_ref(&mut s1);
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn run_deref_coercion_base() {
		deref_coercion_base();
	}

	#[test]
	fn run_deref_coercion_my_smart_pointer() {
		deref_coercion_my_smart_pointer();
	}

	#[test]
	fn run_deref_coercion_deref_operator_immutable() {
		deref_coercion_deref_operator_immutable();
	}

	#[test]
	fn run_deref_coercion_deref_operator_mutable() {
		deref_coercion_deref_operator_mutable();
	}
}
