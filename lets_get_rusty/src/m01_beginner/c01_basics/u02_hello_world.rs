//! # Hello, World!

pub fn hello_world() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_hello_world() {
        hello_world();
    }
}
