use std::{collections::BTreeMap, ops::AddAssign};
use num::traits::{CheckedAdd, CheckedSub, Zero, One};

// This is the System Pallet
// It handles low level state needed for blockchain
#[derive(Debug)]
pub struct Pallet<AccountId, BlockNumber, Nonce> {
    pub block_number: BlockNumber,
    pub nonce: BTreeMap<AccountId, Nonce>,
}

impl<AccountId, BlockNumber, Nonce> Pallet<AccountId, BlockNumber, Nonce>
where
    AccountId: Ord + Clone,
    BlockNumber: Zero + One + CheckedAdd + CheckedSub + Copy + AddAssign,
    Nonce: Ord + Clone + Copy + Zero + One,
{
    pub fn new() -> Self {
        Self {
            block_number: BlockNumber::zero(),
            nonce: BTreeMap::new(),
        }
    }

    pub fn block_number(&self) -> BlockNumber {
        self.block_number
    }
    // Function to increment the block number 
    pub fn inc_block_number(&mut self) {
        // Crashes if overflow by design
        self.block_number += BlockNumber::one();
    }

    pub fn inc_nonce(&mut self, who: &AccountId) {
        let nonce = *self.nonce.get(who).unwrap_or(&Nonce::zero());
        self.nonce.insert(who.clone(), nonce + Nonce::one());
    }

    pub fn get_nonce(&self, who: &AccountId) -> Nonce {
        *self.nonce.get(who).unwrap_or(&Nonce::zero())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_system_pallet() {
        let mut system = Pallet::<String, u32, u32>::new();
        assert_eq!(system.block_number(), 0);

        system.inc_block_number();
        assert_eq!(system.block_number(), 1);

        let alice = "alice".to_string();
        system.inc_nonce(&alice);
        assert_eq!(system.get_nonce(&alice), 1);
    }
}