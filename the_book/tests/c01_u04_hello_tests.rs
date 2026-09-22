use the_book::c01_introduction::u04_hello_test::add_two;
mod common;

#[test]
fn it_adds_two() {
	println!("> test: 'add_two(3)'");
	common::setup();

	let r1 = add_two(3);
	println!("> test: result '${r1}'");
	assert_eq!(5, r1)
}
