use lgr_tests_integration::Account;

mod utils;

#[test]
fn should_have_a_starting_balance_of_0() {
    utils::setup();

    let account = Account::new();
    println!(" > main: {account:?}");

    assert_eq!(0, account.get_balance())
}
