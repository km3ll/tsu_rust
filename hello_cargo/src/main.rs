// Rng: trait
use rand::Rng;

fn main() {

    // Printing values
    let x: i32 = 5;
    let y: i32 = 10;
    println!("153: x = {x} and y + 2 = {}", y + 2);

    // Random number, local to current thread
    let random = rand::thread_rng()
        .gen_range(1..=100);
    println!("153: random value is: {random}");

}