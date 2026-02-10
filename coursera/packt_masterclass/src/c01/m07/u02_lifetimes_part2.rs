//! # Lifetimes - Part 2

// Using the shorter of the two lifetimes
fn longest<'a>(first: &'a str, second: &'a str) -> &'a str {
    if first.len() >= second.len() {
        first
    } else {
        second
    }
}

#[derive(Debug)]
struct User<'a> {
    id: i32,
    name: &'a str,
}

fn lifetimes() {
    let n1 = r#"
    pod: Generic Lifetime Parameter
    - Imposes a lifetime constraint on the references and return values of a function
    - Needed when we use references as output of a function
    ---"#;
    println!("{n1}");

    println!("Lifetimes");
    let s1 = longest("Hello", "Ferris");
    println!(" > longest: s1: {s1}");
}

fn lifetimes_structs() {
    let name = String::from("Ferris");
    let mut user = User {
        id: 1100,
        name: &name,
    };
    /*
    {
        let new_name = String::from( "Ferris");
        user.name = &new_name; // Error: `new_name` does not live long enough
    }
     */

    println!("Lifetimes");
    println!(" > user id: {}, name: {}", user.id, user.name);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_lifetimes() {
        lifetimes()
    }

    #[test]
    fn run_lifetimes_structs() {
        lifetimes_structs()
    }
}
