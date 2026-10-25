#[derive(Debug)]
pub struct Account {
    balance: i32,
}

impl Account {
    /// Creates a new account with a balance of 0
    ///
    /// # Examples
    ///
    /// ```
    /// use lgr_tests_integration::Account;
    /// let account = Account::new();
    /// assert_eq!(account.get_balance(), 0);
    /// ```
    pub fn new() -> Account {
        println!(" > asoc: new()");
        Account { balance: 0 }
    }

    pub fn get_balance(&self) -> i32 {
        println!(" > impl: get_balance()");
        self.balance
    }

    pub fn deposit(&mut self, amount: i32) {
        println!(" > impl: deposit()");
        if amount < 0 {
            panic!(" > cannot deposit a negative amount")
        }
        self.balance += amount;
    }

    pub fn transfer(&self, acc_number: u32, amount: i32) -> Result<String, String> {
        println!(" > impl: transfer()");
        if amount < 0 {
            return Err(String::from(" > cannot transfer a negative amount"));
        }
        Ok(format!(" > transferred ${amount} to {acc_number}"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_have_a_starting_balance_of_0() {
        let account = Account::new();
        assert_eq!(account.get_balance(), 0);
    }

    #[test]
    fn should_be_able_to_deposit() {
        let mut account = Account::new();
        account.deposit(100);
        assert_eq!(account.get_balance(), 100, "Balance should be 100!");
    }

    #[test]
    #[should_panic]
    fn should_panic_if_deposit_is_negative() {
        let mut account = Account::new();
        account.deposit(-1);
    }

    #[test]
    fn should_transfer_non_negative_value() {
        let mut account = Account::new();
        account.deposit(100);
        let res = account.transfer(1100, 100);
        assert!(res.is_ok())
    }

    #[test]
    fn should_not_transfer_negative_value() {
        let mut account = Account::new();
        account.deposit(100);
        let res = account.transfer(1100, -520);
        assert!(res.is_err())
    }
}
