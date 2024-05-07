pub fn main_slices() {
    
    let tweet: String = String::from(
        "This is my tweet and it's very long"
    );
    
    let trimmed_tweet: &str = &tweet[..20];
    println!("trimmed tweet: {trimmed_tweet}");

}