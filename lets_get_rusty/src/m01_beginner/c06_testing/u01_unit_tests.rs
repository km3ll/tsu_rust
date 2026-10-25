//! # Unit Tests

#[derive(Debug)]
pub struct Account {
    balance: i32,
}

impl Account {
    pub fn new() -> Account {
        println!(" > impl: new()");
        Account { balance: 0 }
    }

    pub fn get_balance(&self) -> i32 {
        println!(" > impl: get_galance()");
        self.balance
    }

    pub fn deposit(&mut self, amount: i32) {
        println!(" > impl: deposit()");
        if amount < 0 {
            panic!(" > impl: cannot deposit a negative amount!")
        }
        self.balance += amount
    }

    pub fn transfer(&self, acc_number: u32, amount: i32) -> Result<String, String> {
        println!(" > impl: transfer()");
        Ok(format!(
            " > impl: transferred ${amount} to account number {acc_number}"
        ))
    }
}

fn private_function() {
    println!(" > func: private function")
}

fn tests_definition() {
    let n1 = r#"
    ---
	pod: Test Module
	- Configuration attribute in module: `#[cfg(test)]`
	- Only compile this module when running `cargo test`
	- The convention is to have a `tests` module (child module) in each file you're testing
	- Import items from parent module: `use super::*;`
	- Has access to all items (even private) in the parent module
	---"#;
    println!("{n1}");
}

#[test]
fn run_stand_alone() {
    let n1 = r#"
    ---
    pod: Stand-Alone Tests
    - They could be defined outside the `test` module
    ---"#;
    println!("{n1}");

    let result = 2 + 2;
    assert_eq!(result, 4)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn run_it_works() {
        println!("Unit Tests");
        let result = 2 + 2;
        println!(" > 2 + 2");
        assert_eq!(result, 4)
    }

    #[test]
    fn run_tests_definition() {
        tests_definition();
    }

    #[test]
    fn should_have_a_starting_balance_of_zero() {
        println!("Unit Tests");
        // given
        let account = Account::new();
        // when
        assert_eq!(0, account.get_balance())
    }

    #[test]
    fn should_be_able_to_deposit() {
        let n1 = r#"
        ---
		macro: `assert!()`
		- A custom failure message is allowed as second parameter
		- `assert`
		- `assert_eq!`
		- `assert_ne!`
		---"#;
        println!("{n1}");

        println!("Unit Tests");
        let mut account = Account::new();
        println!(" > main: a1: {account:?}");

        account.deposit(100);

        assert_eq!(100, account.get_balance());
        assert_ne!(0, account.get_balance());
        assert_ne!(
            0,
            account.get_balance(),
            " > main: balance should not be Zero"
        );
    }

    #[test]
    fn should_transfer_money() -> Result<(), String> {
        let n1 = r#"
		---
		pod: Question Mark Operator (`?`)
		- Propagates errors

		pod: Test Cases
		- Can return a result variant such as `Ok()`
		---"#;
        println!("{n1}");

        println!("Unit Tests");
        let mut account = Account::new();
        println!(" > main: a1: {account:?}");

        account.deposit(100);
        account.transfer(123456, 50)?;

        Ok(())
    }

    #[test]
    #[should_panic]
    fn should_panic_if_deposit_is_negative() {
        let n1 = r#"
        ---
		pod: Should Panic attribute
		- Testing that a functions panics
		- `#[should_panic]`
		---"#;
        println!("{n1}");

        println!("Unit Tests");
        let mut account = Account::new();
        println!(" > main: mut a1: {account:?}");

        account.deposit(-1);
    }

    #[test]
    fn should_test_a_private_function() {
        println!("Unit Tests");
        println!(" > main:");
        private_function();
    }
}
