/*
enum Result<T, E> {
    Ok(T),
    Err(E)
}
*/

fn main() {
    println!("Hello, world!");
}

fn get_username(user_id: u32) -> Option<String> {
    let query: String = format!("GET username FROM users WHERE id={user_id}");
    let db_result = query_db(query);
    // The 'ok' method converts a Result enum into an Option enum
    db_result.ok()
}

fn query_db(query: String) -> Result<String, String> {
    if (query.is_empty()) {
        Err(String::from("Query string is empty"))
    } else {
        Ok(String::from("Ferris"))
    }
}