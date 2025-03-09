use crate::m1_beginner::c3_data_types::u7_result::result;

pub fn option() {
    println!("----------");
    println!("Option");

    /**
     * Option Enum
     * - Defined and loaded in the prelude
     */
    let name1 = get_username(1);
    match name1 {
        Some(name) => println!(" > name1: {}", name),
        None => println!(" > name1 not found")
    }

    println!("if-let syntax");
    if let Some(name2) = get_username(1) {
        println!(" > name2: {}", name2);
    }
}

fn get_username(id: u32) -> Option<String> {
    let result = String::from("Ferris");
    if (id == 1) {
        Some(result)
    } else {
        None
    }
}