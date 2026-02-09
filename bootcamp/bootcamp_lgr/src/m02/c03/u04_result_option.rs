//! # Result and Option

use std::{fs, io};

fn read_first_line_v1(filename: &str) -> Result<Option<String>, io::Error> {
    // mismatched error
    // fs::read_to_string(filename).and_then(|s| {
    //   s.lines().next().map(|s| s.to_owned())
    // })
    fs::read_to_string(filename).map(|s| s.lines().next().map(|s| s.to_owned()))
}

fn read_first_line_v2(filename: &str) -> Option<String> {
    // mismatched error
    // fs::read_to_string(filename).and_then(|s| {
    //   s.lines().next().map(|s| s.to_owned())
    // })
    fs::read_to_string(filename)
        .ok()
        .and_then(|s| s.lines().next().map(|s| s.to_owned()))
}

fn result_and_option_with_map() {
    let n1 = r#"
    pod: Closure
    - Anonymous function that you can bass into other functions
    ---
    pod: Combinator
    - Function that performs operations on a value and can even change the value
    - Allows to chain function calls such  as: `and_then()`, `map()` and `ok()`
    ---"#;
    println!("{n1}");

    let r1: Result<Option<String>, io::Error> = read_first_line_v1("example.txt");
    println!("r1: {:?}", r1)
}

fn result_and_option_with_ok_and_then() {
    let r2: Option<String> = read_first_line_v2("example.txt");
    println!("r2: {:?}", r2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_result_and_option_with_map() {
        result_and_option_with_map();
    }

    #[test]
    fn run_result_and_option_with_ok_and_then() {
        result_and_option_with_ok_and_then();
    }
}
