//! # Conditional If and Its Variants

fn conditional() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_conditional() {
        conditional()
    }
}
