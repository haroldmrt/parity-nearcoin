use ethereum_types::{Address};
use types::location::Coordinates;
use std::result::Result;
use error::Error;


/// Single endorser
pub struct Endorser {
    a: Address,
    loc: Coordinates,
    radius: u16,
}

/// Endorser set
/// TODO: use HashMap (or another collection) instead ov Vec 
/// (this implementation is very slow)
pub struct EndorserSet {
    set: Vec<Endorser>,
}

impl EndorserSet {
    /// General constructor
    /// For now endorser 0x7fd56769443e390145a66f0c540d848b54610848 is hardcoded
    pub fn new() -> EndorserSet {
        EndorserSet {
            set: vec![
                Endorser {
                    a: Address::from("7fd56769443e390145a66f0c540d848b54610848"),
                    loc: Coordinates::from_raw(0x1000, 0x2000),
                    radius: 1000u16,
                },
            ],
        }
    }
    pub fn is_endorser(&self, a: &Address) -> bool {
        // TODO: replace Vec by HashMap
        for v in &self.set {
            if &v.a == a {
                return true;
            }
        }
        return false;
    }

    pub fn add_endorser(&mut self, a: &Address, loc: &Coordinates, radius: u16) -> Result<(), Error> {
        let v = Endorser {
            a: a.clone(),
            loc: loc.clone(),
            radius: radius,
        };
        self.set.push(v);
        Ok(())
    }

    pub fn can_validate(&self, endorser: &Address, loc: &Coordinates) -> bool {
        // TODO: replace Vec by HashMap
        // let endorser_location;
        // let endorser_radius;
        // for v in &self.set {
        //     if &v.a == endorser {
        //         endorser_location = &v.loc;
        //         endorser_radius = v.radius;
        //     }
        // }

        // if loc.distance(endorser_location) < endorser_radius {
        //     true
        // } else {
        //     false
        // }
        true
    }
}

impl Clone for EndorserSet {
    fn clone(&self) -> EndorserSet {
        EndorserSet {
            set: self.set.clone(),
        }
    }
}

impl Clone for Endorser {
    fn clone(&self) -> Endorser {
        Endorser {
            a: self.a.clone(),
            loc: self.loc.clone(),
            radius: self.radius.clone(),
        }
    }
}