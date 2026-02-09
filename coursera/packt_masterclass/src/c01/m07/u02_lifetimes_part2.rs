//! # Lifetimes - Part 2

fn lifetimes() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_lifetimes() {
        lifetimes()
    }
}
