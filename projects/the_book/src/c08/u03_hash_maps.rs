use std::collections::HashMap;

fn load_scores() -> HashMap<String, i32> {
	let mut scores: HashMap<String, i32> = HashMap::new();
	scores.insert(String::from("Blue"), 10);
	scores.insert(String::from("Yellow"), 50);
	scores
}

fn hashmap_definition() {
	let n1 = r#"
	pod: Hash Maps
	- Keys have the same type
	- Values have the same type
	- Types that implement the 'Copy' trait are copied into the hash map
	- Owned values are moved and the hash map is the owner
	- Update Operations
	  - Overwrite (insert)
	  - Adding key-value only if key isn't present
	  - Updating a value based on old value
	- Iterating over a hash map happens in an arbitrary order
	---"#;
	println!("{n1}");

	let mut scores = load_scores();
	println!("Hashmaps: scores: {scores:?}");
}

fn hashmap_values() {
	let mut scores = load_scores();
	println!("Hashmaps: scores: {scores:?}");

	let s1 = scores.get("Blue").copied().unwrap_or(0);
	println!("> Blue: {s1}");

	let s2 = scores.get("Green").copied().unwrap_or(0);
	println!("> Green: {s2}");

	println!("Hashmaps: scores loop:");
	for (key, value) in &scores {
		println!("> {key}: {value}");
	}
}

fn hashmap_ownership() {
	let k1 = String::from("Orange");
	let v1 = 100;
	let mut scores: HashMap<String, i32> = HashMap::new();
	scores.insert(k1, v1);

	println!("Hashmaps: owned values of scores: {scores:?}");
	// println!("Hashmaps: k1: {k1}"); Error: Value used after being moved
}

fn hashmap_updating() {
	let mut scores = load_scores();
	println!("Hashmaps: updating:");
	scores.insert(String::from("Blue"), 1000);
	println!("> v1: {scores:?}");

	scores.entry(String::from("Green")).or_insert(9999);
	println!("> v2: {scores:?}");

	scores.entry(String::from("Blue")).or_insert(9999);
	println!("> v3: {scores:?}");
}

fn hashmap_old_value() {
	let text = "hello world wonderful world";
	let mut map: HashMap<&str, i32> = HashMap::new();

	for word in text.split_whitespace() {
		let count = map.entry(word).or_insert(0);
		*count += 1;
	}
	println!("Hashmaps: map-reduce: {map:?}");
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn run_hashmap_definition() {
		hashmap_definition();
	}

	#[test]
	fn run_hashmap_values() {
		hashmap_values();
	}

	#[test]
	fn run_hashmap_ownership() {
		hashmap_ownership();
	}

	#[test]
	fn run_hashmap_updating() {
		hashmap_updating();
	}

	#[test]
	fn run_hashmap_old_value() {
		hashmap_old_value();
	}
}
