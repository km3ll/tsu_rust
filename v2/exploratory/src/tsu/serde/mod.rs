use serde::{Deserialize, Serialize};
use serde_json;
use serde_xml_rs;

#[derive(Serialize, Deserialize, Debug)]
struct User {
	id: u32,
	name: String,
}

pub fn serdes_json() {
	let user = User {
		id: 1,
		name: String::from("John Wick"),
	};
	let serialized = serde_json::to_string(&user).unwrap();
	println!("Ser JSON: {}", serialized);

	let deserialized: User = serde_json::from_str(&serialized).unwrap();
	println!("Des JSON: {:?}", deserialized);
}

pub fn serdes_xml() {
	let user = User {
		id: 1,
		name: String::from("John Wick"),
	};
	let serialized = serde_xml_rs::to_string(&user).unwrap();
	println!("Ser XML: {}", serialized);

	let deserialized: User = serde_xml_rs::from_str(&serialized).unwrap();
	println!("Des XML: {:?}", deserialized);
}
