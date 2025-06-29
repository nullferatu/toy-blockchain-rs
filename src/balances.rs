use std::collections::BTreeMap;
use num::traits::{CheckedAdd, CheckedSub, Zero};

pub trait Config: crate::system::Config {
    type Balance: Zero + CheckedSub + CheckedAdd + Copy;
}

#[derive(Debug)]
pub struct Pallet<T: Config> {
    pub balances: BTreeMap<T::AccountId, T::Balance>,
}

#[macros::call]
impl <T: Config> Pallet<T> {
    // Transfer 'amount' from one account to another
    pub fn transfer(
        &mut self,
        caller: T::AccountId,
        to: T::AccountId,
        amount: T::Balance,
    ) -> Result<(), &'static str> {
        let caller_balance = self.balance(&caller);
        let to_balance = self.balance(&to);

        let new_caller_balance = caller_balance
            .checked_sub(&amount)
            .ok_or("Insufficient balance")?;
        
        let new_to_balance = to_balance
            .checked_add(&amount)
            .ok_or("Overflow when adding to balance")?;

        self.set_balance(&caller, new_caller_balance);
        self.set_balance(&to, new_to_balance);

        Ok(())
    }
}

impl <T: Config> Pallet<T> {
    pub fn new() -> Self {
        Self {
            balances: BTreeMap::new(),
        }
    }

    // Set the balance of an account 'who' to some 'amount'
    pub fn set_balance(&mut self, who: &T::AccountId, amount: T::Balance) {
        self.balances.insert(who.clone(), amount);
    }

    // Get the balance of an account 'who'
    pub fn balance(&self, who: &T::AccountId) -> T::Balance {
        *self.balances.get(who).unwrap_or(&T::Balance::zero())
    }
}

#[cfg(test)]
mod tests {
    use crate::system;
    struct TestConfig;
    impl system::Config for TestConfig {
        type AccountId = String;
        type BlockNumber = u32;
        type Nonce = u32;
    }

    impl super::Config for TestConfig {
        type Balance = u128;
    }

    #[test]
    fn init_balances() {
        let mut balances: super::Pallet<TestConfig> = super::Pallet::new();

        assert_eq!(balances.balance(&"alice".to_string()), 0);
        balances.set_balance(&"alice".to_string(), 100);
        assert_eq!(balances.balance(&"alice".to_string()), 100);
        assert_eq!(balances.balance(&"bob".to_string()), 0);
    }

    #[test]
    fn transfer_balance() {
        let alice = "alice".to_string();
        let bob = "bob".to_string();

        let mut balances: super::Pallet<TestConfig> = super::Pallet::new();
        balances.set_balance(&alice, 100);
        let _ = balances.transfer(alice.clone(), bob.clone(), 90);

        assert_eq!(balances.balance(&alice), 10);
        assert_eq!(balances.balance(&bob), 90);
    }

    #[test]
    fn balance_insufficient() {
        let alice = "alice".to_string();
        let bob = "bob".to_string();
        
        let mut balances: super::Pallet<TestConfig> = super::Pallet::new();

        let result = balances.transfer(alice.clone(), bob.clone(), 110);

        assert_eq!(result, Err("Insufficient balance"));
        assert_eq!(balances.balance(&"alice".to_string()), 0);
        assert_eq!(balances.balance(&"bob".to_string()), 0);
    }
}