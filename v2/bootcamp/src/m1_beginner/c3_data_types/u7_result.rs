pub fn result() {
    println!("----------");
    println!("Result");

    /**
     * Result Enum
     * - Included in Rust prelude
     */
    let result1 = query_db(String::from(""));
    println!(" > result1: {:#?}", result1);

    let result2 = query_db(String::from("GET username FROM users WHERE id = 1"));
    println!(" > result2: {:#?}", result2);
}

fn get_username(id: u32) -> Option<String> {
    let query = format!("GET username FROM users WHERE id = {id}");
    let result: Result<String, String> = query_db(query);
    // Converts Result into Option
    result.ok()
}

fn query_db(query: String) -> Result<String, String> {
    if query.is_empty() {
        Err(String::from("Query is empty!"))
    } else {
        Ok(String::from("Ferris"))
    }
}