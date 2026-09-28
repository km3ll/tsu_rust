pub struct SavingsAccount {
    balance: i32,
}

// Doc comments are written in markdown and support code blocks/ 
// Some common sections included in documentation blocks are: 
// an 'example' section to show examples, a 'panic' section to
// explaing why a function might panic, and a 'failure' section
// explaining what type of failures your function can return.

// Cargo executes documentation code blocks as tests, so that you
// can be sure that the example code compiles. These tests are 
// displayed within section Doc-tests

/// A savings account
impl SavingsAccount {

    /// Creates a `SavingsAccount` with balance of 0
    /// 
    /// # Examples
    /// 
    /// ```
    /// use c3_documentation::SavingsAccount;
    /// let account = SavingsAccount::new();
    /// assert_eq!(account.get_balance(), 0);
    /// ```
    pub fn new() -> SavingsAccount {
        SavingsAccount { balance: 0 }
    }

    pub fn get_balance(&self) -> i32 {
        self.balance
    }

    pub fn credit(&mut self, amount: i32) {
        if amount < 0 {
            panic!("Cannot credit a negative amount!");
        }
        self.balance += amount
    }

    pub fn transfer(&self, acc_number: u32, amount: i32) -> Result<String, String> {
        Ok(format!("Transferred ${amount} to ${acc_number}"))
    }
}


pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn private_add(left: usize, right: usize) -> usize {
    left + right
}


// Only compile this module when running tests via 'cargo test'
// Unit tests are written along the source code. The convention is to have one tests module
// in each file you are testing. The content could be inline or in a separate file.
#[cfg(test)]
mod tests {

    // Because tests is a child module we need to import all the items from the parent module
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn should_have_a_starting_balance_of_zero() {
        let account = SavingsAccount::new();
        assert_eq!(account.get_balance(), 0);
    }

    #[test]
    fn should_be_able_to_credit() {
        let mut account = SavingsAccount::new();
        account.credit(200);

        // All these three macros will panic if the assertion fails
        assert_eq!(account.get_balance(), 200);
        assert_ne!(account.get_balance(), 0);
        assert!(account.get_balance() == 200, "Balance should be $200 USD");

    }

    // Test functions could also return a result enum
    // This is tipicallt used when the tested function returns a result type itself
    #[test]
    fn should_transfer_money() -> Result<(), String> {
        let mut account = SavingsAccount::new();
        account.credit(100);

        // We can use the question mark operation to propagate errors
        account.transfer(123456, 100)?;
        Ok(())
    }

    // We can also check that a function panics
    #[test]
    #[should_panic]
    fn should_panic_if_credit_is_negative() {
        let mut account = SavingsAccount::new();
        account.credit(-1);
    }

    // We can also test private functions
    // tests is a child module and child modules have access to private items in their parent module
    #[test]
    fn private_function_should_be_reachable() {
        let result = private_add(2, 2);
        assert_eq!(result, 4);
    }

}

// Test functions could be standalone
// However, the convention is to create a module called tests
#[test]
fn it_still_works() {
    let result = add(5, 10);
    assert_eq!(result, 15);
}