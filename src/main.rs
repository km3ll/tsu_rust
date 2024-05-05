#![allow(unused)]
mod beginner_m01_setup;
mod rust_pl;

//use crate::rust_pl::c01_pod::*;
use crate::beginner_m01_setup::c05_functions::*;

fn main() {
    let age: i32 = get_age();
    print_age(age);
}