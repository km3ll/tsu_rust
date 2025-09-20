/**
 * pod: Doc comments
 * - Start with three slashes ///
 * - Are written in markdown and support code blocks
 * - Common sections included in doc blocks are:
 *   - Examples
 *   - Panic
 *   - Failure
 * - Cargo runs code in doc blocks as tests
 */
pub fn documentation_() {
	println!("Base");
}

/// A savings account
pub struct Account {
	balance: i32,
}

impl Account {
	/// Creates an `Account` with balance of zero
	///
	/// # Examples
	///
	/// ```
	/// let account = Account::new();
	/// assert_eq!(0, account.get_balance())
	/// ```
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

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn documentation_() {}
}
	