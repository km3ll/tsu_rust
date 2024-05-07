use unicode_segmentation::UnicodeSegmentation;

pub fn main_strings() {
    
    // stored in the application's binary
    let s1: &str = "Привіт Світ! 🦀";
    // store in the heap
    let s2: String = String::from("Привіт Світ!");

    let s3: String = "Привіт Світ!".to_string();
    let s4: String = "Привіт Світ!".to_owned();
    
    let s5: &str = &s4[..];
    println!("{}", s5);

    let mut greeting: String = String::from("Hello ");
    greeting.push_str("Ferris 🦀");
    println!("{greeting}");

    greeting.replace_range(.., "Greetings, pod");
    println!("{greeting}");

    // + operator moves values
    let name: String = String::from("John ");
    let last_name: String = String::from("Wick");
    
    let full_name: String = name + &last_name;
    // value of name was moved
    // println!("{}", name); // Error: borrow of moved value
    println!("{}", full_name);

    // The format macro copies the values of the strings
    // It can also receive string slices
    let st1: String = String::from("tic");
    let st2: String = String::from("tac");

    let game: String = format!("{}-{}-{}", st1, st2, "toe");
    println!("{}", game);

    // other concatenation examples
    let s6: String = ["first", "-", "second"].concat();
    println!("{}", s6);

    let s7: String = format!("{}-{}", "first", "second");
    println!("{}", s7);

    let s8: &str = concat!("first", "-", "second");
    println!("{}", s8);

    let s9: String = String::from("first-");
    let s10 = s9 + "second";
    println!("{}", s10);

    // indexing
    let emojis: &str = "🦀🦀🦀🦀🦀";
    /*
    Error: type str cannot be indexed by integer.
    A string is a collection of bytes, so emojis[0] would give us the first byte
    In UTF-8 a character can be from 1 to 4 bytes. Ferris the crab is 4 bytes long
    let first_emoji = emojis[0];
    */

    // We have to know exactly how many bytes a character is
    let first_emoji: &str = &emojis[0..4];
    println!("first emoji: {}", first_emoji);

    // iterating over the bytes
    println!("bytes in namaste: ");
    for b in "नमस्ते".bytes() {
        println!("{b} ");
    }

    // iterating over the characters
    println!("characters in namaste: ");
    for c in "नमस्ते".chars() {
        println!("{c} ");
    }
    
    // In Unicode, user perceiverd characters are known as 'grapheme clusters' 
    // iterating over the graphemes
    println!("graphemes in namaste: ");
    for g in "नमस्ते".graphemes(true) {
        println!("{g} ");
    }

    // Deref Coercion
    // We can pass a string slice or a string reference to our function
    // When we pass a reference it's autmatically coerced to a string slice
    let s11: &str = "Hello";
    let s12: String = String::from("Hello");
    my_format(s11);
    my_format(&s12);

}

// We want to returns a String type because we want the caller to take ownership
fn my_format(s: &str) -> String {
    println!("my format: {s}");
    return format!("{}", s);
}