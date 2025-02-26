// Mute warnings
#![allow(unused)]

mod m1_beginner;

use crate::m1_beginner::c3_data_types::u1_structs::*;
use crate::m1_beginner::c3_data_types::u2_impl_blocks::*;
use crate::m1_beginner::c3_data_types::u3_tuple_structs::*;
use crate::m1_beginner::c3_data_types::u4_enums::*;
use crate::m1_beginner::c3_data_types::u5_matching::*;
use crate::m1_beginner::c3_data_types::u6_option::*;
use crate::m1_beginner::c3_data_types::u7_result::*;
use crate::m1_beginner::c3_data_types::u8_vectors::*;

use crate::m1_beginner::c2_memory_safety::u1_memory_regions::*;
use crate::m1_beginner::c2_memory_safety::u2_ownership_part1::*;
use crate::m1_beginner::c2_memory_safety::u3_ownership_part2::*;
use crate::m1_beginner::c2_memory_safety::u4_borrowing::*;
use crate::m1_beginner::c2_memory_safety::u5_slices::*;
use crate::m1_beginner::c2_memory_safety::u6_strings::*;

use crate::m1_beginner::c1_get_started::u1_hello_pod::*;
use crate::m1_beginner::c1_get_started::u2_variables::*;
use crate::m1_beginner::c1_get_started::u3_data_types::*;
use crate::m1_beginner::c1_get_started::u4_constants::*;
use crate::m1_beginner::c1_get_started::u5_functions::*;
use crate::m1_beginner::c1_get_started::u6_flow_control::*;
use crate::m1_beginner::c1_get_started::u7_comments::*;

fn main() {
	module1_chapter1();
	module1_chapter2();
	module1_chapter3();
}

fn module1_chapter3() {
	// u1
	structs();
	// u2
	implementation_blocks();
	// u3
	tuple_structs();
	// u4
	enums();
	// u5
	matching();
	// u6
	option();
	// u7
	result();
	// u8
	vectors()
	
}

fn module1_chapter2() {
	// u1
	memory_regions();
	// u2
	ownership_part1();
	// u3
	ownership_part2();
	// u4
	borrowing();
	// u5
	slices();
	// u6
	strings();
}

fn module1_chapter1() {
	// u1
	hello_pod();
	// u2
	variables();
	// u3
	data_types();
	// u4
	constants();
	// u5``
	functions();
	// u6
	flow_control();
	// u7
	comments();
}