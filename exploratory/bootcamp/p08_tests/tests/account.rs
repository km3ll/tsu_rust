use p08_tests::Account;

use crate::utils::common_setup;

/**
 * pod: Child test modue
 * - Utils can be used
 */
mod utils;

/**
 * pod: Integration tests
 * - Defined in root's tests folder
 * - External to the library
 * - Test the public interface of our library
 * - Test the interaction between multiple units of code
 * - Every file in the tests directory is treated as a separate crate
 * - The downside is that is hard to share code (utils.rs)
 */
#[test]
fn should_have_a_starting_balance_of_zero() {
    let account = Account::new();
    assert_eq!(0, account.get_balance())
}

/**
 * pod: Importing code from binary crate
 * - Integration tests cannot import items from a binary crate directly
 * - Common pattern to have a small binary crate plusft a library crate,
 *   which contains most of the functionality for integration tests
 */
#[test]
fn should_use_common_setup_from_utils() {
    common_setup();
}
