//! # Declarative Macros Part 1

fn base() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_() {
        base();
    }
}
