//! # Generics

#[derive(Debug)]
struct Command<T> {
    name: String,
    payload: T,
}

impl<T> Command<T> {
    fn new(name: String, payload: T) -> Self {
        println!(" > asoc<T>: new()");
        Command {
            name: name,
            payload: payload,
        }
    }

    fn get_payload(&self) -> &T {
        println!(" > impl<T>: get_payload()");
        &self.payload
    }
}

impl Command<String> {
    fn print_payload(&self) {
        println!(" > impl<String> payload: {}", self.payload)
    }
}

pub fn generics() {
    let n1 = r#"
    ---
    pod: Generics Conventions
    - Start in `T` (uppercase), which represents `Type`
    - Use `T + Integer`, as in `T0`, `T1`, etc.
    - Use Camel Case as in PayloadType

    pod: Implementation Blocks
    - For any command type: `impl<T> Command<T> {}`

    pod: Self Return Type
    - If we change the name of the struct, we don't have to change the return type

    pod: Generics In Methods
    - `fn get_payload(&self) -> &T {}`

    pod: Implementation Block
    - For a concrete type

    pod: Monomorphization
    - Rust resolves generics at compile time as concrete implementations
    ---"#;
    println!("{n1}");
}

pub fn generics_structs() {
    println!("Generics & Structs");

    let cmd1: Command<String> = Command {
        name: "navigate".to_owned(),
        payload: "www.google.com".to_owned(),
    };
    println!(" > cmd1: {cmd1:?}");

    let cmd2: Command<i32> = Command {
        name: "zoom".to_owned(),
        payload: 200,
    };
    println!(" > cmd2: {cmd2:?}");
}

pub fn generics_impl_block_generic() {
    println!("Generics & Impl Blocks");

    println!(" > main:");
    let cmd3: Command<String> = Command::new("navigate".to_owned(), "www.google.com".to_owned());
    println!(" > main: cmd3: {cmd3:?}");

    let cmd4: Command<i32> = Command::new("zoom".to_owned(), 200);
    println!(" > main: cmd4: {cmd4:?}");
}

pub fn generics_impl_block_concrete() {
    println!("Generics & Concrete Impl Block");
    println!(" > main:");
    let cmd5: Command<String> = Command::new("navigate".to_owned(), "www.google.com".to_owned());
    println!(" > main: cmd5: {cmd5:?}");
    cmd5.print_payload();
}

pub fn generics_method() {
    println!("Generics & Methods");
    println!(" > main:");

    let cmd5: Command<String> = Command {
        name: "navigate".to_owned(),
        payload: "www.google.com".to_owned(),
    };
    println!(" > main: cmd5: {cmd5:?}");
    let pl5: &String = cmd5.get_payload();
    println!(" > main: payload: {pl5}");

    let cmd6: Command<i32> = Command {
        name: "zoom".to_owned(),
        payload: 200,
    };
    println!(" > main: cmd6: {cmd6:?}");
    let pl6: &i32 = cmd6.get_payload();
    println!(" > main: payload: {pl6}");
}

pub fn generics_emum() {
    println!("Generics & Enums");
    enum Option<T> {
        Some(T),
        None,
    }
    let e1 = Some("Hello".to_owned());
    println!(" > e1: {e1:?}");

    let e2 = Some(1100);
    println!(" > e2: {e2:?}");
}

fn serialize_payload<T>(payload: T) -> String {
    println!(" > free: serialize_payload()");
    // Convert into JSON string
    "{ \"payload\": \"\"}".to_owned()
}

pub fn generics_free_function() {
    println!("Generics & Free Function");
    let cmd7: Command<i32> = Command {
        name: "zoom".to_owned(),
        payload: 200,
    };
    println!(" > main: cmd7: {cmd7:?}");

    let pl7: &i32 = cmd7.get_payload();
    println!(" > main: payload: {pl7:?}");

    let ser1 = serialize_payload(pl7);
    println!(" > main: serialized: {ser1}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_generics() {
        generics()
    }

    #[test]
    fn run_generics_structs() {
        generics_structs();
    }

    #[test]
    fn run_generics_impl_block_generic() {
        generics_impl_block_generic();
    }

    #[test]
    fn run_generics_impl_block_concrete() {
        generics_impl_block_concrete();
    }

    #[test]
    fn run_generics_method() {
        generics_method();
    }

    #[test]
    fn run_generics_enum() {
        generics_emum();
    }

    #[test]
    fn run_generics_free_function() {
        generics_free_function();
    }
}
