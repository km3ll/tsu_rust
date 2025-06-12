#![allow(unused)]

mod c01_getting_started;
mod c02_guessing_game;

use c01_getting_started::c01u03_hello_cargo::*;
use c02_guessing_game::c02u01_guessing_game::*;

fn main() {
	chapter_02()
}

// 1. Getting Started
fn chapter_01() {
	// 1.3. Hello, Cargo!
	hello_cargo();
}

// 2. Programming a Guessing Game
fn chapter_02() {
	start_game();
}