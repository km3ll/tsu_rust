#![allow(unused)]

use serde::Serialize;
// Generics allow you to define structs, enums and functions with
// generic types that will be substituted for concrete types at copile time

// The convention is to name generics with an uppercase letter starting up
// with T, which represents type. Another convention is to use T and an
// integer number, for example: T0, T1...

// 1. Structs
#[derive(Serialize, Debug)]
struct BrowserCommand<T> {
    name: String,
    payload: T,
}

// 2. Implementation Blocks
// The reason we need to declare T type twice is so that Rust knows that we're implementing
// functionality for the struct of any type rather than a concrete type.
impl<T> BrowserCommand<T> {
    // The return type us Self, which is going to be BrowserCommand.
    // Using Self is convenient because if we change the name of the
    // struct we don't have to change the return type.
    fn new(name: String, payload: T) -> Self {
        BrowserCommand {
            name,
            payload
        }
    }

    fn get_payload(&self) -> &T {
        &self.payload
    }

}

// Implementation Block for a Concrete type
// You can create multiple implementation blocks on the same type
impl BrowserCommand<String> {
    fn print_payload(&self) {
        println!("Payload is: {}", self.payload);
    }
}

// 3. Enums
enum Option<T> {
    Some(T),
    None,
}

enum Result<T, E> {
    Ok(T),
    Err(E),
}

// 4. Free Functions
// Functions that are not tied to structs, enums or traits
fn serialize_payload<T>(payload: T) -> String {
    "placeholder".to_owned()
}


// 5. Monomorphization
// At compile time, Rust will take a generic function and create two concrete
// functions: one that takes a string and one that takes an integer. Then, Rust
// will update the call sites of the generic function to call the concrete
// versions. This process also happens with structs, enums and implementation
// blocks

fn main() {

    let cmd1: BrowserCommand<String> = BrowserCommand {
        name: "navigate".to_owned(),
        payload: "https://www.letsgetrusty.com".to_owned(),
    };
    println!("cmd1 = {:?}", cmd1);
    cmd1.print_payload();

    let cmd2: BrowserCommand<i32> = BrowserCommand::new(
        "zoom".to_owned(),
        200
    );
    println!("cmd2 = {:?}", cmd2);

    let payload1 = cmd1.get_payload();
    println!("payload1 = {:?}", payload1);

    let payload2 = cmd2.get_payload();
    println!("payload = {:?}", payload2);

    serialize_payload("test-drive".to_owned());
    serialize_payload(30);

}