pub fn slices() {
    println!("----------");
    println!("Slices");

    /**
     * Slices
     * - References to a contiguous sequence of elements in a collection
     * - Used to reference part of a collection
     */

    let tweet: String = String::from("This is my tweet and it's very long.");
    println!("tweet: {tweet}");

    println!("Range syntax");
    let s1: &str = &tweet[..20]; // beginning
    let s2: &str = &tweet[20..]; // ending
    let s3: &str = &tweet[..];   // entire
    println!("s1: {s1}");
    println!("s2: {s2}");
    println!("s3: {s3}");

    /**
     * &str
     * - Immutable sequence of UTF-8 bytes somewhere in memory
     *   (stack, heap, or static memory)
     * - Handle behind a reference (&str) because length of
     *   sequence is unknown at compile time.
     */
    println!("&str");
    let t1: &str = &tweet[..20];
    println!("t1: {t1}");

    /**
     * String literals
     * - all string literals are string slices
     */
    println!("String literals");
    let s4: &str = "Ferris the crab";
    println!("s4: {s4}");

    /** 
     * String slices and functions
     */
    println!("Functions");
    let s5: &str = trim_tweet(&tweet);
    println!("s5: {s5}");

    /**
     * Dref Cohersion
     * - Function works for both: reference to string and string slice
     */
    println!("Dref Cohersion");
    let s6: &str = trim_tweet_v2(&tweet);
    let s7: &str = trim_tweet_v2("This is my tweet and it's very long.");
    println!("s6: {s6}");
    println!("s7: {s7}");

    /** Vector slices */
    println!("Vector slices");
    let v1: [i32; 6] = [1, 2, 3, 4, 5, 6];
    let v2: &[i32] = &v1[..3];
    println!("{:?}", v1); // {:?} syntax for debug formatting
    println!("{:?}", v2);
}

fn trim_tweet(tweet: &String) -> &str {
    &tweet[..16]
}

fn trim_tweet_v2(tweet: &str) -> &str {
    &tweet[..16]
}