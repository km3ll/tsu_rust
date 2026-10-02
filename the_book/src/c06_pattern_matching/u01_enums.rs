//! # Defining an Enum

#[derive(Debug)]
enum IpKindV1 {
	V4,
	V6,
}

#[derive(Debug)]
enum IpKindV2 {
	V4(u8, u8, u8, u8),
	V6(String),
}

#[derive(Debug)]
struct IpAddr {
	kind: IpKindV1,
	address: String,
}

#[derive(Debug)]
struct IpV4Addr {
	value: String,
}

#[derive(Debug)]
struct IpV6Addr {
	value: String,
}

#[derive(Debug)]
enum IpKindV3 {
	V4(IpV4Addr),
	V6(IpV6Addr),
}

#[derive(Debug)]
enum Message {
	Quit,
	Move { x: i32, y: i32 },
	Write(String),
}

impl Message {
	fn call(&self) {
		println!(" > variant: {self:?}");
	}
}

fn caller(message: &Message) {
	message.call()
}

fn route(ip_kind: IpKindV1) {
	println!(" > IpKindV1: {ip_kind:?}");
}

fn enums() {
	let n1 = r#"
	---
	pod: Enums
	- Enumerations define a type by enumerating its possible variants
	- An enum value can only be one of its variants
	- We can put data into enum variants: strings, numeric types, enums, or structs
	- The name of each enum becomes a function that constructs an instance of the enum
	- We can also define methods on structs using `impl blocks`
	---"#;
	println!("{n1}");

	println!("Enums");
	let ip1 = IpKindV1::V4;
	println!(" > ip1: {ip1:?}");
}

fn enums_in_structs() {
	println!("Enums in structs");
	let home = IpAddr {
		kind: IpKindV1::V4,
		address: String::from("127.0.0.1"),
	};
	let loopback = IpAddr {
		kind: IpKindV1::V6,
		address: String::from("::1"),
	};
	println!(" > home: {home:?}");
	println!(" > loopback: {loopback:?}");
}

fn enums_with_values() {
	println!("Enums with values");
	let home = IpKindV2::V4(127, 0, 0, 1);
	let loopback = IpKindV2::V6(String::from("::1"));
	println!(" > home: {home:?}");
	println!(" > loopback: {loopback:?}");
}

fn enums_with_structs() {
	println!("Enums with structs");
	let ip1 = IpV4Addr {
		value: String::from("172.0.0.1"),
	};
	let home = IpKindV3::V4(ip1);
	println!(" > home: {home:?}");
}

fn enums_variants() {
	println!("Enums with variants");
	let m1 = Message::Quit;
	caller(&m1);
	let m2 = Message::Move { x: -10, y: 20 };
	caller(&m2);
	let m3 = Message::Write(String::from("Success"));
	caller(&m3);
}

fn enums_option() {
	let n1 = r#"
	---
	pod: Option<T> Type
	- A value could be something or could be nothing
	- It's included in the prelude
	- Rust doesn't have the `null` feature
	- A null is a value that is currently invalid or absent for some reason
	---"#;
	println!("{n1}");

	println!("Option type");
	let op1: Option<i32> = Some(5);
	println!(" > op1: {op1:?}");

	let op2: Option<i32> = None;
	println!(" > op2: {op2:?}");
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn run_enums() {
		enums();
	}

	#[test]
	fn run_enums_in_structs() {
		enums_in_structs();
	}

	#[test]
	fn run_enums_with_values() {
		enums_with_values();
	}

	#[test]
	fn run_enums_with_structs() {
		enums_with_structs();
	}

	#[test]
	fn run_enums_variants() {
		enums_variants();
	}

	#[test]
	fn run_enums_option() {
		enums_option();
	}
}
