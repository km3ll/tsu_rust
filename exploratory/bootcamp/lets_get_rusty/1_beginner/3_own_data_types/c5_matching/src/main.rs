fn main() {

    // Match is a control-flow operator that allows you to compare a value
    // against a series of patterns to determine which code path to execute.
    let age: i32 = 38;
    match age {
        1 => println!("Happy 1st Birthday"),
        13..=19 => println!("You are a teenager"),
        x => println!("You are {x} years old")
    }

}
