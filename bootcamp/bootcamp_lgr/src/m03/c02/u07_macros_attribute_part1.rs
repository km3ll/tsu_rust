//! # Procedural Macros - Attribute Like

fn base() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_() {
        base();
    }
}
