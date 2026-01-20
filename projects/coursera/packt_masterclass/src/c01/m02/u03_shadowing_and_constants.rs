fn variables_definition() {
    let (is_active, ferris) = (true, '🦀');
    println!("Variables: grouped definition: ({}, {})", is_active, ferris);

    let large_number: i64 = 1_000_000_000_000_000;
    println!("Variables: large number: {}", large_number);
}

fn variables_bases() {
    let n1 = r#"
    pod: Bases of Numbers
    - Octal (:o), Binary (:b) and Hexa (:x)
    ---"#;
    println!("{n1}");

    let i1 = 300;
    println!("Variables: bases of '300'");
    println!(" > octal: {:o}", i1);
    println!(" > binary: {:b}", i1);
    println!(" > hexadecimal: {:x}", i1);
}

fn variables_convert() {
    let n1: i32 = 14;
    let n2: f64 = 15.5;

    let n3: i32 = n1 + n2 as i32;
    println!("Variables: convert as i32 n3: {n3}\n > loss of value");

    let n4 = n1 as f64 + n2;
    println!("Variables: convert as f64: n4: {n4}");
}

fn shadowing() {
    let n1 = r#"
    pod: Shadowing
    - Using, updating or declaring a variable with the same name which has been previously used or declared
    - The first variables is being shadowed by the second using:
      - The 'let' keyword
      - A mutable variable by an immutable variable
      - A change in data type
      - A code segment (scope limited to the segment)
    ---"#;
    println!("{n1}");

    let s1 = 2;
    let s1 = 20;
    println!("Shadowing: s1: {s1}");

    let mut s2 = 3;
    let s2 = 30;
    println!("Shadowing: mutable s2: {s2}");

    let s3: i32 = 4;
    let s3: char = '🦀';
    println!("Shadowing: data type: s3: {s3}");

    let s4 = 5;
    println!("Shadowing scopes: ");
    println!(" > outer: s4: {s4}");
    {
        let s4 = 50;
        println!(" > inner: s4: {s4}");
    }
    println!(" > outer: s4: {s4}");
}

fn constants() {
    let n1 = r#"
    pod: Constants
    - Data values that remain the same, not changed, every time the program executes
    - Keyword 'mut' is not allowed
    - Require explicit type definition (not auto-inferred)
    ---"#;
    println!("{n1}");

    const MAX_RANGE: u32 = 20000;
    println!("Constants: MAX_RANGE: {MAX_RANGE}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_variables_definition() {
        variables_definition()
    }

    #[test]
    fn run_variables_bases() {
        variables_bases()
    }

    #[test]
    fn run_variables_convert() {
        variables_convert()
    }

    #[test]
    fn run_shadowing() {
        shadowing()
    }

    #[test]
    fn run_constants() {
        constants()
    }
}
