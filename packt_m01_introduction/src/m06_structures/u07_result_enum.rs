//! # Result Enum

fn divide_v1(dividend: f64, divisor: f64) -> Result<f64, String> {
    println!(" > {dividend} / {divisor}");
    if divisor == 0.0 {
        Err(String::from("Divisor cannot be Zero"))
    } else {
        Ok(dividend / divisor)
    }
}

fn divide_v2(dividend: f64, divisor: f64) -> Result<f64, String> {
    println!(" > {dividend} / {divisor}");
    match divisor {
        0.0 => Err(String::from("Divisor cannot be Zero")),
        _ => Ok(dividend / divisor),
    }
}

fn result_enum() {
    let n1 = r#"
    ---
    pod: Result Enum
    - Represents either success or failure
    - Variants: `Ok(T)`, `Err(E)`
    ---"#;
    println!("{n1}");

    println!("Result Enum");

    let d1 = divide_v1(10.0, 2.0);
    println!(" > d1: {d1:?}");

    let d2 = divide_v1(10.0, 0.0);
    println!(" > d2: {d2:?}");

    let d3 = divide_v2(20.0, 10.0);
    println!(" > d3: {d3:?}");

    let d4 = divide_v2(20.0, 0.0);
    println!(" > d4: {d4:?}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_result_enum() {
        result_enum()
    }
}
