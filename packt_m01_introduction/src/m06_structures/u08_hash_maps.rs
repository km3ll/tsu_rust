//! # Hash Maps

use std::collections::HashMap;

fn hash_maps() {
    let n1 = r#"
    ---
    pod: Mash Maps
    - A storage of key-value pairs (a dictionary)
    - Keys are unique with no duplicates
    ---"#;
    println!("{n1}");

    println!("HashMap");
    let mut names: HashMap<&str, u32> = HashMap::new();
    names.insert("Ferris", 30);
    names.insert("John", 31);
    println!(" > names: {names:?}");
}

fn hash_map_functions() {
    println!("HashMap");
    let mut colors: HashMap<&str, u32> = HashMap::new();
    println!(" > colors: {colors:?}");

    println!(" > insert");
    colors.insert("Yellow", 10);
    colors.insert("Blue", 20);
    colors.insert("Red", 30);
    colors.insert("Green", 40);
    println!(" > colors: {colors:?}");

    println!(" > get Green: {:?}", colors.get("Green"));
    println!(" > contains Gray: {}", colors.contains_key("Gray"));

    println!(" > for-loop:");
    for color in &colors {
        println!(" > {color:?}");
    }

    let key = "Blue";
    let value = 9999;
    println!(" > insert {key}={value} (override)");
    colors.insert(key, value);
    println!(" > get {key}: {:?}", colors.get(key));

    println!(" > entry_or_insert: Black=8888");
    colors.entry("Black").or_insert(8888);
    println!(" > get Black: {:?}", colors.get("Black"));
}

fn hash_map_from_vector() {
    println!("HashMap");
    let vector: Vec<i32> = vec![5, 5, 8, 8, 1, 0, 1, 5, 5, 5, 5];
    println!(" > vector: {vector:?}");

    println!(" > entry().or_insert()");
    let mut frequency: HashMap<i32, u32> = HashMap::new();
    for element in &vector {
        println!(" > e: {element}");
        let freq: &mut u32 = frequency.entry(*element).or_insert(0);
        *freq += 1;
    }
    println!(" > frequency: {:?}", frequency);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_hash_maps() {
        hash_maps()
    }

    #[test]
    fn run_hash_map_functions() {
        hash_map_functions()
    }

    #[test]
    fn run_hash_map_from_vector() {
        hash_map_from_vector()
    }
}
