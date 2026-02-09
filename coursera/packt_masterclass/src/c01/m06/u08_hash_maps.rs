//! # Hash Maps

use std::collections::HashMap;

fn hash_maps() {
    let n1 = r#"
    pod: Mash Maps
    - A storage of key-value pairs (a dictionary)
    - Keys are unique with no duplicates
    ---"#;
    println!("{n1}");

    let mut names: HashMap<&str, u32> = HashMap::new();
    names.insert("Ferris", 30);
    names.insert("John", 31);
    println!("HashMap");
    println!(" > names: {names:?}");
}

fn hash_map_functions() {
    let mut colors: HashMap<&str, u32> = HashMap::new();
    colors.insert("Yellow", 10);
    colors.insert("Blue", 20);
    colors.insert("Red", 30);
    colors.insert("Green", 40);

    println!("HashMap");
    println!(" > get Green: {:?}", colors.get("Green"));
    println!(" > contains: {}", colors.contains_key("Gray"));

    println!(" > for-loop:");
    for color in &colors {
        println!("   > {color:?}");
    }

    colors.insert("Green", 9999);
    println!(" > insert: Green: {:?}", colors.get("Green"));

    colors.entry("Grey").or_insert(8888);
    println!(" > entry_or_insert: Green: {:?}", colors.get("Green"));
}

fn hash_map_from_vector() {
    let vector: Vec<i32> = vec![5, 5, 8, 8, 1, 0, 1, 5, 5, 5, 5];
    let mut frequency: HashMap<i32, u32> = HashMap::new();

    for element in &vector {
        let freq: &mut u32 = frequency.entry(*element).or_insert(0);
        *freq += 1;
    }

    println!("Hashmaps");
    println!(" > vector: {:?}", vector);
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
