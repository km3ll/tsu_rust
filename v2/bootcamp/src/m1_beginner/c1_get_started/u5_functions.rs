pub fn functions() {
    println!("----------");
    println!("Functions");
    
    my_function(20);
    
    let r1: i32 = plus_two(98);
    println!("plus_two returned: {}", r1);

    let r2: i32 = times_two(100);
    println!("times_two returned: {}", r2);   
}

pub fn my_function(x: i32) {
    // Statements are instructions that do not return a value
    println!("my_function called with: {}", x);
}

pub fn plus_two(x: i32) -> i32 {
    // Expressions are code that evaluate to a value
    let y: i32 = x + 2;

    // For a function to use the last expression as return type,
    // we have to omit the semicolon. This syntax only works for
    // the last expression in a function
    y
}

pub fn times_two(x: i32) -> i32 {
    let y: i32 = x * 2;
    // To return early we use the return keyword + a semi-colon.
    return y;
}