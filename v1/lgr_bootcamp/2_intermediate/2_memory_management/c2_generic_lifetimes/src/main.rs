#![allow(unused)]

fn main() {
    
    game_v1();
    game_v2();
    game_v3();
    game_v4();

}

pub fn game_v4() {

    println!("Starting game v4!");

    let player1: String = String::from("player 1");
    let result;

    {
        let player2: String = String::from("player 2");
        result = first_turn_v2(player1.as_str(), player2.as_str()); 
        
    }
    println!("Player going first is: {}", result);
    
}

pub fn game_v3() {

    println!("Starting game v3!");

    let player1: String = String::from("player 1");
    //let result;

    {
        let player2: String = String::from("player 2");
        // Error: player2 does not live enough to be borrowed
        // result = first_turn(player1.as_str(), player2.as_str()); 
        
    }
    //println!("Player going first is: {}", result);
    
}

pub fn game_v2() {

    println!("Starting game v2!");

    let player1: String = String::from("player 1");
    {
        let player2: String = String::from("player 2");
        let result: &str = first_turn_v1(player1.as_str(), player2.as_str());
        println!("Player going first is: {}", result);
    }
    
}

pub fn game_v1() {

    println!("Starting game v1!");

    let player1: String = String::from("player 1");
    let player2: String = String::from("player 2");
    
    let result: &str = first_turn_v1(player1.as_str(), player2.as_str());

    println!("Player going first is: {}", result);

}

// Perspective of the borrow checker
// The lifetime of the returned reference is ambiguous. 
// It could be p1 or p2 and both can have different lifetimes.

// Error: lifetime specifier.
// Lifetime specifiers also known as generic lifetime annotations are a way to describe
// a relationship between lifetime of references. They are not concrete lifetimes, but 
// rather describe the relationship between concrete lifetimes.

// Generics of lifetimes are defined with a tick as prefix. The convention is to name your
// lifetime starting off with a (lowercase) and then going with the alphabet.. 

// Here we are saying that there is a relationshionship between p1, p2 and the returned value.
// The lifetime of the returned value is going to be equal to the shortest lifetime passed in.
fn first_turn_v1<'a>(p1: &'a str, p2: &'a str) -> &'a str {
    if rand::random() {
        p1
    } else {
        p2
    }
}

// The lifetime of the first value is going to be equal to the lifetime of p1
fn first_turn_v2<'a>(p1: &'a str, p2: &str) -> &'a str {
    p1
}

// Static lifetimes define references that can live for the entire duration of the program,
// String slices have a static lifetime because they live in the program's binary, meaning
// that they're valid for the entire duration of the program. This means we can return 
// string slices created inside of functions.

// In the function signature we are saying the that return value has a lifetime that is 
// equivalent to p1. This works because we are saying that our return value must live at
// least as long as p1, and because s has a static lifetime, it will be greater or 
// equal to p1
fn first_turn_v3<'a>(p1: &'a str, p2: &str) -> &'a str {
    let s = "Let's Get Rusty!";
    s
}

// A more accurate definition would be
fn first_turn_v4(p1: &str, p2: &str) -> &'static str {
    let s = "Let's Get Rusty!";
    s
}