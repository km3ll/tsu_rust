//! # Propagating Errors

use std::{
    fs::File,
    io::{self, Read},
};

struct User {
    first_name: String,
    last_name: String,
}

fn get_initials(user: User) -> Option<String> {
    let first_initial = user.first_name.chars().next()?;
    let last_initial = user.last_name.chars().next()?;
    Some(format!("{first_initial}.{last_initial}."))
}

fn read_file_v1(name: &str) -> Result<String, io::Error> {
    let mut file: File = File::open(name)?;
    // pod: Empty heap-allocated String
    let mut contents: String = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn read_file_v2(name: &str) -> Result<String, io::Error> {
    // pod: in-line error propagation
    let mut contents: String = String::new();
    File::open(name)?.read_to_string(&mut contents)?;
    Ok(contents)
}

fn propagating_errors_question_mark_v1() {
    let n1 = r#"
    pod: Question Mark Operator (`?`)
    - Unwraps valid values or returns erroneous values propagating them to the calling function
    - Also works with Optional type
    ---"#;
    println!("{n1}");

    let r1 = read_file_v1("example.txt");
    println!("r1: {:?}", r1)
}

fn propagating_errors_question_mark_v2() {
    let r2 = read_file_v2("example.txt");
    println!("r2: {:?}", r2)
}

fn propagating_errors_option() {
    let u1 = User {
        first_name: String::from("John"),
        last_name: String::from("Wick"),
    };
    let s1 = get_initials(u1);
    println!("s1: {:?}", s1);

    let u2 = User {
        first_name: String::from("John"),
        last_name: String::from(""),
    };
    let s2 = get_initials(u2);
    println!("s2: {:?}", s2);

    let u3 = User {
        first_name: String::from(""),
        last_name: String::from("Wick"),
    };
    let s3 = get_initials(u3);
    println!("s3: {:?}", s3);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_propagating_errors_question_mark_v1() {
        propagating_errors_question_mark_v1();
    }

    #[test]
    fn run_propagating_errors_question_mark_v2() {
        propagating_errors_question_mark_v2();
    }

    #[test]
    fn run_propagating_errors_option() {
        propagating_errors_option();
    }
}
