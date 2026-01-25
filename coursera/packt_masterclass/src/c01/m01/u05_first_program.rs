//! # Running and Compiling Your First Program

fn first_program() {
    println!("Hello, pod!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_first_program() {
        first_program();
    }
}
