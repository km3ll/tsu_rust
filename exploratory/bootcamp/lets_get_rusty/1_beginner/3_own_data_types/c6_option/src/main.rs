use rand::Rng;

fn main() {
    
    let user_id: u32 = rand::thread_rng().gen_range(1..=5);
    let username: Option<String> = get_username(user_id);
    
    /*
    match username {
        Some(name) => println!("Hello {}", name),
        None => println!("User with ID = {} not found", user_id)
    }
    */

    // 'if let' syntax
    if let Some(name) = username {
        println!("Hello again {}", name)
    }

}

fn get_username(user_id: u32) -> Option<String> {
    let db_result: String = String::from("Ferris");
    if user_id == 3 {
        Some(db_result)
    } else {
        None
    }
}