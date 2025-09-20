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

    // pod: panic! macro
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

/**
 * pod: Configuration attribute
 * - Only compile this module when running 'cargo test'
 * - This is a child module
 * - The convention is to have a 'tests' module in each file you're testing
 */
#[cfg(test)]
mod tests {

    // pod: Import items from parent module
    use super::*;

    #[test]
    fn run_it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4)
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

        // pod: assert_eq! macro
        assert_eq!(100, account.get_balance());

        // pod: assert_ne! macro
        assert_ne!(0, account.get_balance());

        // pod: custom failure message
        // pod: assert_ne! macro
        assert_ne!(0, account.get_balance(), "Balance should not be Zero");
    }

    #[test]
    fn should_transfer_money() -> Result<(), String> {
        let mut account = Account::new();
        account.deposit(100);

        /**
         * pod: Error variant
         * - Question mark operator (?) propagates errors
         */
        // pod: Error variant
        account.transfer(123456, 50)?;

        /**
         * pod: Ok variant
         * - Instead of using assert macros we return the Ok()
         */
        // pod:
        Ok(())
    }

    /**
     * pod: #[should_panic] attribute
     * - Testing that a function panics
     */
    #[test]
    #[should_panic]
    fn should_panic_if_deposit_is_negative() {
        let mut account = Account::new();
        account.deposit(-1);
    }

    /**
     * pod: Testing a private function
     * - 'tests' is a regular child module
     * - it has access to all items (even private) in the parent module
     */
    #[test]
    fn should_test_a_private_function() {
        private_function();
    }
}
