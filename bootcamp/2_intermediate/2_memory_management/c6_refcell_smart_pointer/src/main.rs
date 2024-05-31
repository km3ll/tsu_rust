#![allow(unused)]
use std::rc::Rc;
use std::cell::RefCell;

struct Database {
    max_connections: u32
}

struct AuthService {
    db: Rc<RefCell<Database>>
}

struct ContentService {
    db: Rc<RefCell<Database>>
}

fn main() {

    let db = Rc::new(RefCell::new(Database { 
        max_connections: 100 
    }));
    
    let auth_service = AuthService { db: Rc::clone(&db) };
    let content_service = ContentService { db: Rc::clone(&db) };
    
    println!("Started with {} max connections", db.borrow().max_connections);

    // The Rc smart pointer only allows immutable shared ownership of a value.
    // db.max_connections = 200; Error: cannot assign to data in a Rc trait

    // Refcell uses the internal mutability design pattern, which allows us to mutably
    // borrow data even if there's an immutable reference to that data.
    db.borrow_mut().max_connections = 200;
    println!("Continued with {} max connections", db.borrow().max_connections);

    /*
    thread main panicked at 'already borrowed'
    let mut r1 = db.borrow_mut();
    let r2 = db.borrow_mut();
    r1.max_connections = 300
    */

}