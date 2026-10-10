//! # Control Flow
use rand::Rng;

pub fn flow_if_else() {
    println!("If-Else");
    let a1 = rand::rng().random_range(-40..40);
    if a1 > 30 {
        println!(" > if: Bigger than 30");
    } else if 1 > 20 {
        println!(" > else if: Bigger than 20");
    } else {
        println!(" > else: Smaller or equal to 20")
    }
}

pub fn flow_if_else_in_let() {
    let n1 = r#"
    ---
	pod: If-Else Expressions
	- Can be used in let statements
	---"#;
    println!("{n1}");

    println!("If-Else Let");
    let a1: i8 = rand::rng().random_range(-100..=100);
    let b1 = if a1 > 0 { "positive" } else { "negative" };
    println!(" > a1: {a1}, let b1: {b1}")
}

pub fn flow_loop() {
    let n1 = r#"
    ---
	pod: Loop
    - inner / outer
	- `break`
	- `break` + `'name` (starts with tick `'`)
	- `brear` + value
	---"#;
    println!("{n1}");

    println!("Loop");
    loop {
        println!(" > break");
        break;
    }
}

pub fn flow_labeling_loops() {
    println!("Loop Label");
    println!(" > outer: 'name");
    'name: loop {
        println!(" > inner");
        loop {
            println!(" > inner: break 'name");
            break 'name;
        }
    }
}

pub fn flow_loop_returning_value() {
    println!("Loop Value");
    let res: i8 = loop {
        println!(" > break 5");
        break 5;
    };
    println!(" > res: {res}");
}

pub fn flow_while_loop() {
    println!("While Loop");
    let mut i: i8 = 1;
    println!("> loop i: {i}");
    while i <= 3 {
        println!(" > + 1");
        i = i + 1;
    }
    println!("> loop i: {i}");
}

pub fn flow_for_loop() {
    println!("For Loop");
    let arr1: [i8; 4] = [10, 20, 30, 40];
    println!(" > arr1: {arr1:?}");
    for e in arr1 {
        println!(" > e: {e}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_flow_ifelse() {
        flow_if_else();
    }

    #[test]
    fn run_flow_ifelse_in_let() {
        flow_if_else_in_let();
    }

    #[test]
    fn run_flow_loop() {
        flow_loop();
    }

    #[test]
    fn run_flow_labeling_loops() {
        flow_labeling_loops();
    }

    #[test]
    fn run_flow_loop_returning_value() {
        flow_loop_returning_value();
    }

    #[test]
    fn run_flow_while_loop() {
        flow_while_loop();
    }

    #[test]
    fn run_flow_for_loop() {
        flow_for_loop();
    }
}
