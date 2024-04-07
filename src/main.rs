// pod: disables the warning over unused function
#![allow(unused)]

use crate::c01_pod::start_pod;
use crate::c02_game::start_game;
use crate::c03_concepts::e01_mutability;
use crate::c03_concepts::e02_constants;

mod c01_pod;
mod c02_game;
mod c03_concepts;

fn main() {
    e02_constants();
    //e01_mutability();
    //start_game();
    //start_pod();
}