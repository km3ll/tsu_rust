//! # Rc Smart Pointer

use std::rc::Rc;

struct Database {}

struct AuthService {
    db: Rc<Database>,
}

struct ContentService {
    db: Rc<Database>,
}

fn rc_smart_pointer_moved() {
    let n1 = r#"
    pod: Reference Counting Smart Pointer (Rc)
    - Shared ownership
    - Not included in the prelude
    - Can only be used in single-threaded applications
    - Only allows immutable shared ownership of a value
    ---
    pod: Rc::clone()
    - Doesn't actually clone, instead it increments the reference count
    ---"#;
    println!("{n1}");

    let db = Database {};
    // let auth_service = AuthService { db: db };
    // let content_service = AuthService { db: db };
    println!("error: use of moved value 'db'");
}

fn rc_smart_pointer_shared_reference() {
    let db: Rc<Database> = Rc::new(Database {}); // Rc: 1
    let auth_service = AuthService { db: Rc::clone(&db) }; // Rc: 2
    let content_service = AuthService { db: Rc::clone(&db) }; // Rc: 3

    println!("Reference count is 3");
} // Rc is dropped

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_rc_smart_pointer_moved() {
        rc_smart_pointer_moved();
    }

    #[test]
    fn run_rc_smart_pointer_shared_reference() {
        rc_smart_pointer_shared_reference();
    }
}
