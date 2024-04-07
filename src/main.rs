// pod: disables the warning over unused function
#![allow(unused)]

use crate::c01_pod::start_pod;
use crate::c02_game::start_game;
use crate::c03_concepts::*;

mod c01_pod;
mod c02_game;
mod c03_concepts;

fn main() {
    constants();
    //mutability();
    //start_game();
    //start_pod();
}