// pod: disables the warning over unused function
#![allow(unused)]

mod game;

// pod: std = standard
use rand::Rng;
use std::io;

// pod: the main function is always the first code that runs.
fn main() {
    execute();
}

fn execute() {
    println!("pod 042: start");
    loop {
        let mut cmd: String = String::new();
        io::stdin()
            .read_line(&mut cmd)
            .expect("Failed to read command");
        let cmd: &str = cmd.as_str().trim();
        if cmd == "thanks" {
            break;
        } else {
            run_cmd(cmd);
            continue;
        }
    }
    println!("pod 042: stop");
}

fn run_cmd(cmd: &str) {
    match cmd {
        "mantra" => mantra(),
        _ => println!(" > Unrecognized command '{cmd}'")
    }
}

fn mantra() {
    let mantras: [&str; 3] = [
        "No one should be honored for doing what is expected.",
        "The best is yet to come.",
        "Information is just an opportunity: it means nothing if you don't use it."
    ];
    let n: usize = rand::thread_rng()
        // pod: range expression is inclusive on the lower and upper bounds
        .gen_range(0..=1);
    println!(" > {}", mantras[n])
}