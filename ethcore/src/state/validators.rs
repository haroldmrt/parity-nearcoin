use ethereum_types::{Address, U256};
use types::location::Coordinates;

pub struct Validator {
    a: Address,
    loc: Coordinates,
    radius: U256,
}

pub struct ValidatorSet {
    set: Vec<Validator>,
}

impl ValidatorSet {
    pub fn new() -> ValidatorSet {
        ValidatorSet {
            set: vec![],
        }
    }
    pub fn is_validator(&self, a: &Address) -> bool {
        true
    }

    pub fn add_validator(&mut self, a: &Address, loc: Coordinates, radius: U256) -> bool {
        false
    }
}

impl Clone for ValidatorSet {
    fn clone(&self) -> ValidatorSet {
        ValidatorSet {
            set: self.set.clone(),
        }
    }
}

impl Clone for Validator {
    fn clone(&self) -> Validator {
        Validator {
            a: self.a.clone(),
            loc: self.loc.clone(),
            radius: self.radius.clone(),
        }
    }
}