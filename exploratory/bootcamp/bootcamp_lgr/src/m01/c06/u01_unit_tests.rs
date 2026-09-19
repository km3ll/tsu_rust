//! # Unit Tests

pub struct Account {
    balance: i32,
}

impl Account {
    pub fn new() -> Account {
        Account { balance: 0 }
    }

    pub fn get_balance(&self) -> i32 {
        self.balance
    }

    pub fn deposit(&mut self, amount: i32) {
        if amount < 0 {
            panic!("Cannot deposit a negative amount!")
        }
        self.balance += amount
    }

    pub fn transfer(&self, acc_number: u32, amount: i32) -> Result<String, String> {
        Ok(format!(
            "Transferred ${amount} to account number {acc_number}"
        ))
    }
}

fn private_function() {
    println!("Private function")
}

fn tests_definition() {
    let n1 = r#"
	pod: Stand-Alone Tests
	- They could be defined outside the `test` module
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
    let result = 2 + 2;
    assert_eq!(result, 4)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn run_it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4)
    }

    #[test]
    fn run_tests_definition() {
        tests_definition();
    }

    #[test]
    fn should_have_a_starting_balance_of_zero() {
        // given
        let account = Account::new();
        // when
        assert_eq!(0, account.get_balance())
    }

    #[test]
    fn should_be_able_to_deposit() {
        let mut account = Account::new();
        account.deposit(100);

        let n1 = r#"
		pod: macro: assert!
		- assert
		- assert_eq!
		- assert_ne!
		- A custom failure message is allowed as second parameter
		---"#;
        println!("{n1}");

        assert_eq!(100, account.get_balance());
        assert_ne!(0, account.get_balance());
        assert_ne!(0, account.get_balance(), "Balance should not be Zero");
    }

    #[test]
    fn should_transfer_money() -> Result<(), String> {
        let mut account = Account::new();
        account.deposit(100);

        let n1 = r#"
		pod: Question Mark Operator (`?`)
		- Propagates errors
		---"#;
        println!("{n1}");

        account.transfer(123456, 50)?;

        let n1 = r#"
		pod: Test Cases
		- Can return a result variant such as `Ok()`
		---"#;
        println!("{n1}");
        Ok(())
    }

    #[test]
    #[should_panic]
    fn should_panic_if_deposit_is_negative() {
        let n1 = r#"
		pod: Should Panic attribute
		- `#[should_panic]`
		- Testing that a functions panics
		---"#;
        println!("{n1}");

        let mut account = Account::new();
        account.deposit(-1);
    }

    #[test]
    fn should_test_a_private_function() {
        private_function();
    }
}
