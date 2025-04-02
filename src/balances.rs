use std::collections::BTreeMap;
use num::traits::{CheckedAdd, CheckedSub, Zero};

#[derive(Debug)]
pub struct Pallet<AccountId, Balance> {
    pub balances: BTreeMap<AccountId, Balance>,
}

impl <AccountId, Balance> Pallet<AccountId, Balance> 
where
    AccountId: Ord + Clone,
    Balance: Zero + CheckedAdd + CheckedSub + Copy,
{
    pub fn new() -> Self {
        Self {
            balances: BTreeMap::new(),
        }
    }

    // Set the balance of an account 'who' to some 'amount'
    pub fn set_balance(&mut self, who: &AccountId, amount: Balance) {
        self.balances.insert(who.clone(), amount);
    }

    // Get the balance of an account 'who'
    pub fn balance(&self, who: &AccountId) -> Balance {
        *self.balances.get(who).unwrap_or(&Balance::zero())
    }

    // Transfer 'amount' from one account to another
    pub fn trasnfer(
        &mut self,
        caller: AccountId,
        to: AccountId,
        amount: Balance,
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