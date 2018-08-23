use ethereum_types::{Address};
use types::location::Coordinates;
use std::result::Result;
use error::Error;


/// Single validator
pub struct Validator {
    a: Address,
    loc: Coordinates,
    radius: u16,
}

/// Validator set
/// TODO: use HashMap (or another collection) instead ov Vec 
/// (this implementation is very slow)
pub struct ValidatorSet {
    set: Vec<Validator>,
}

impl ValidatorSet {
    /// General constructor
    pub fn new() -> ValidatorSet {
        ValidatorSet {
            set: vec![
                Validator {
                    a: Address::from("7fd56769443e390145a66f0c540d848b54610848"),
                    loc: Coordinates::from_raw(0x1000, 0x2000),
                    radius: 1000u16,
                },
            ],
        }
    }
    pub fn is_validator(&self, a: &Address) -> bool {
        // TODO: replace Vec by HashMap
        for v in &self.set {
            if &v.a == a {
                return true;
            }
        }
        return false;
    }

    pub fn add_validator(&mut self, a: &Address, loc: &Coordinates, radius: u16) -> Result<(), Error> {
        let v = Validator {
            a: a.clone(),
            loc: loc.clone(),
            radius: radius,
        };
        self.set.push(v);
        Ok(())
    }

    pub fn can_validate(&self, validator: &Address, loc: &Coordinates) -> bool {
        // TODO: replace Vec by HashMap
        // let validator_location;
        // let validator_radius;
        // for v in &self.set {
        //     if &v.a == validator {
        //         validator_location = &v.loc;
        //         validator_radius = v.radius;
        //     }
        // }

        // if loc.distance(validator_location) < validator_radius {
        //     true
        // } else {
        //     false
        // }
        true
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