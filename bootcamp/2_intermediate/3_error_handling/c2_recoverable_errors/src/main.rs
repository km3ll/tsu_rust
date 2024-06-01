#![allow(unused)]

use std::{fs::File, io::Write};

fn main() {

    /*
    match get_user_id("") {
        Ok(id) => println!("Success: User id is {}", id),
        Err(e) => println!("Error  : {}", e)
    }
    */

    match get_user_id("johnwick") {
        Ok(id) => println!("Success: User id is {}", id),
        Err(e) => println!("Error  : {}", e)
    }

    let result1: Result<File, std::io::Error> = File::open("example.txt");
    let file: File = match result1 {
        Ok(file) => file,
        Err(error) => {
            panic!("Failed to open file_1: {}", error);
        }
    };

    // unwrap
    let file_2: File = File::open("example.txt").unwrap();

    // expect
    let file_3: File = File::open("example2.txt")
        .expect("Failed to open file_3!");

}

fn get_user_id(username: &str) -> Result<i32, String> {
    if username.is_empty() {
        Err("Username cannot be empty".to_owned())
    } else {
        Ok(1)
    }
}