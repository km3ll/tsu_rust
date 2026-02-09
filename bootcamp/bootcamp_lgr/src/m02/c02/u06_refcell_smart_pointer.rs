//! # RefCell Smart Pointer

use std::cell::RefCell;
use std::rc::Rc;

struct Database {
    max_connections: u32,
}

struct AuthService {
    db: Rc<RefCell<Database>>,
}

struct ContentService {
    db: Rc<RefCell<Database>>,
}

fn ref_cell_smart_pointer_invalid() {
    let n1 = r#"
    pod: Rc Smart Pointer
    - Only allows immutable shared ownership of a value 
    ---"#;
    println!("{n1}");

    let db: Rc<Database> = Rc::new(Database {
        max_connections: 100,
    });
    //let auth_service = AuthService { db: Rc::clone(&db) };
    //let content_service = AuthService { db: Rc::clone(&db) };
    // db.max_connections = 200;
    println!("error: cannot assign to data in a 'Rc' trait, trait 'DerefMut' is required");
}

fn ref_cell_smart_pointer_valid() {
    let n1 = r#"
    pod: RefCell Smart Pointer
    - Uses unsafe Rust
    - We need to call the borrow method before updating
    - Uses the interior mutability design pattern, which allows to mutably borrow data even when there is an immutable reference to that data
    - Used in combination with Rc Smart Pointer we get shared ownership and mutability
    ---"#;
    println!("{n1}");

    let db: Rc<RefCell<Database>> = Rc::new(RefCell::new(Database {
        max_connections: 100,
    }));
    let auth_service = AuthService { db: Rc::clone(&db) };
    let content_service = AuthService { db: Rc::clone(&db) };
    db.borrow_mut().max_connections = 200;
    println!("max connections: {}", db.borrow().max_connections)
}

fn ref_cell_smart_pointer_panic() {
    let db: Rc<RefCell<Database>> = Rc::new(RefCell::new(Database {
        max_connections: 100,
    }));
    let auth_service = AuthService { db: Rc::clone(&db) };
    let content_service = AuthService { db: Rc::clone(&db) };

    // let mut r1 = db.borrow_mut();
    // let mut r2 = db.borrow_mut();
    println!("error: thread 'main' panicked at 'already borrowed: BorrowMutError'");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_ref_cell_smart_pointer_invalid() {
        ref_cell_smart_pointer_invalid();
    }

    #[test]
    fn run_ref_cell_smart_pointer_valid() {
        ref_cell_smart_pointer_valid();
    }

    #[test]
    fn run_ref_cell_smart_pointer_panic() {
        ref_cell_smart_pointer_panic();
    }
}
