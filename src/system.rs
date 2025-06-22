use std::{collections::BTreeMap, ops::AddAssign};
use num::traits::{Zero, One};

pub trait Config {
    type AccountId: Ord + Clone;
    type BlockNumber: Zero + One + AddAssign + Copy;
    type Nonce: Zero + One + Copy;
}

// This is the System Pallet
// It handles low level state needed for blockchain
#[derive(Debug)]
pub struct Pallet<T: Config> {
    pub block_number: T::BlockNumber,
    pub nonce: BTreeMap<T::AccountId, T::Nonce>,
}

impl<T: Config> Pallet<T> {
    pub fn new() -> Self {
        Self {
            block_number: T::BlockNumber::zero(),
            nonce: BTreeMap::new(),
        }
    }

    pub fn block_number(&self) -> T::BlockNumber {
        self.block_number
    }
    // Function to increment the block number 
    pub fn inc_block_number(&mut self) {
        // Crashes if overflow by design
        self.block_number += T::BlockNumber::one();
    }

    pub fn inc_nonce(&mut self, who: &T::AccountId) {
        let nonce = *self.nonce.get(who).unwrap_or(&T::Nonce::zero());
        self.nonce.insert(who.clone(), nonce + T::Nonce::one());
    }

    pub fn get_nonce(&self, who: &T::AccountId) -> T::Nonce {
        *self.nonce.get(who).unwrap_or(&T::Nonce::zero())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    struct TestConfig;
    impl super::Config for TestConfig {
        type AccountId = String;
        type BlockNumber = u32;
        type Nonce = u32;
    }

    #[test]
    fn test_system_pallet() {
        let mut system = Pallet::<TestConfig>::new();
        assert_eq!(system.block_number(), 0);

        system.inc_block_number();
        assert_eq!(system.block_number(), 1);

        let alice = "alice".to_string();
        system.inc_nonce(&alice);
        assert_eq!(system.get_nonce(&alice), 1);
    }
}