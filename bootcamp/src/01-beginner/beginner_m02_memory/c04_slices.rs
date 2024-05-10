pub fn main_slices() {
    
    let tweet: String = String::from(
        "This is my tweet and it's very long"
    );
    
    // range syntax
    // ommiting the begining index
    let trimmed1: &str = &tweet[..20]; // String Slice

    // ommiting the ending index
    let trimmed2: &str = trim_tweet(&tweet); // String Slice

    println!("Trimmed 1: {trimmed1}");
    println!("Trimmed 2: {trimmed2}");

    // String literals are string slices
    // They are stored in your application's binary
    let literal: &str = "my string";

    let tweet2: &str = "This is another long tweet";
    let trimmed_tweet2: &str = trim_tweet_for_both_types(tweet2);

    // other collections
    let a: [i32; 8] = [1, 2, 3, 4, 5, 6, 7, 8];
    let a_slice: &[i32] = &a[..3];

    // syntas :? prints a string with debug formatting
    println!("{:?}", a_slice);

}

fn trim_tweet(tweet: &String) -> &str {
    &tweet[20..]
}

// This function works for strings and string slices
fn trim_tweet_for_both_types(tweet: &str) -> &str {
    &tweet[20..]
}