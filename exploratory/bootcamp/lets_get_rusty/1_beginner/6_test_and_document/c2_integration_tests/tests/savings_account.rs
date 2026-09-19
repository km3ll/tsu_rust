// Cargo will compile each file of the tests directory as a separate crate
use c2_integration_tests::SavingsAccount;

// We can add utils as a child module
mod utils;

// Integration tests cannot import items from a binary crate directly.
// So a common pattern is to have (1) a small binary create and (2) a library crate,
// which contains most of the functionality, which could be integration tested.

#[test]
fn should_have_a_starting_balance_of_zero() {
    utils::common_setup();
    let account = SavingsAccount::new();
    assert_eq!(account.get_balance(), 0);
}