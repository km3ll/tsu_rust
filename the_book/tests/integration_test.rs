use the_book::c01::u04_hello_test::add_two;
mod common;

#[test]
fn it_adds_two() {
	common::setup();
	let result = add_two(3);
	println!("Integration tests: result: {result}");
	assert_eq!(5, result)
}
