#![allow(unused)]
use std::rc::Rc; 

struct Database {}

struct AuthService {
    db: Rc<Database>
}

struct ContentService {
    db: Rc<Database>
}

fn main() {
    
    let db = Rc::new(Database {}); //reference-count = 1

    // We are transferring ownership
    let auth_service = AuthService { db: Rc::clone(&db) }; //reference-count = 2

    // Value db was been moved, but we want to use shared ownership.
    // So, we can use the RC Pointer which stands for Reference Counting
    let content_service = ContentService { db: Rc::clone(&db) }; //reference-count = 3

    // The Rc:clone function will increment the reference count.
    // At the end of main, the reference count is decremented to zero and then destroyed

    // Rc smart pointer can only be used in single-threaded applications
    println!("Hello, world!");

}