#![allow(unused)]
use std::{fs::File, io::{self, Read}};

fn main() {
    
    println!("Reading file...");
    let content: String = read_file("example.txt").unwrap();
    println!("{}", content);

    println!("Getting initials...");
    let user: User = User::new(String::from("John"), String::from("Wick"));
    let initials: String = get_initials(user).unwrap();
    println!("{}", initials);            

}

fn read_file(filename: &str) -> Result<String, io::Error> {

    // The question mark operator (?) unwraps valid values or returns erroneous 
    // values propagating them to the calling function.
    let mut file: File = File::open(filename)?;

    // Heap-Allocated String
    let mut contents: String = String::new();

    // read_to_string returns a result type
    file.read_to_string(&mut contents)?;
    Ok(contents)

    // Another variant
    // File::open(filename)?.read_to_string(&mut contents)?;

}

// The question mark operator also works with optional values
struct User {
    first_name: String, 
    last_name: String
}

impl User {
    fn new(first_name: String, last_name: String ) -> User {
        User { first_name, last_name }
    }
}

fn get_initials(user: User) -> Option<String> {
    let first_initial = user.first_name.chars().next()?;
    let last_initial = user.last_name.chars().next()?;
    Some(format!("{}.{}.", first_initial, last_initial))
}