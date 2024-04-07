// pod: disables the warning over unused function
#![allow(unused)]

use crate::game::start_game;
use crate::pod::start_pod;

mod game;
mod pod;

fn main() {
    //start_game();
    start_pod();
}