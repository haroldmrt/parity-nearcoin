use ethereum_types::{H32};
use rlp;
use std::fmt;
use std::f64::consts::PI;

/// Type containing geographic coordinates as integers
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Coordinates {
    lat: u16,
    lng: u16,
}

impl From<Coordinates> for H32 {
    fn from(coord: Coordinates) -> Self {
        let b1 : u8 = ((coord.lat >> 8) & 0xff) as u8;
        let b2 : u8 = (coord.lat & 0xff) as u8;
        let b3 : u8 = ((coord.lng >> 8) & 0xff) as u8;
        let b4 : u8 = (coord.lng & 0xff) as u8;
        H32::from([b1, b2, b3, b4])
    }
}

impl From<H32> for Coordinates {
    fn from(data: H32) -> Self {
        let bytes = <[u8; 4]>::from(data);
        Coordinates {
            lat: ((bytes[0] as u16) << 8) | bytes[1] as u16,
            lng: ((bytes[2] as u16) << 8) | bytes[3] as u16,
        }
    }
}

impl rlp::Encodable for Coordinates {
    fn rlp_append(&self, s: &mut rlp::RlpStream) {
        let b1 : u8 = ((self.lat >> 8) & 0xff) as u8;
        let b2 : u8 = (self.lat & 0xff) as u8;
        let b3 : u8 = ((self.lng >> 8) & 0xff) as u8;
        let b4 : u8 = (self.lng & 0xff) as u8;
        let tmp : u32 = (b1 as u32) << 24 | (b2 as u32) << 16 | (b3 as u32) << 8 | (b4 as u32);
        s.append_internal(&tmp);
    }
}

impl rlp::Decodable for Coordinates {
    fn decode(rlp: &rlp::Rlp) -> Result<Self, rlp::DecoderError> {
        if rlp.is_empty() {
            // Do we accept coordinates {0, 0} (-90, -180) ?
            // tied to Executive::call
            Err(rlp::DecoderError::Custom("Empty location"))
            // Ok(Coordinates::new())
        } else {
            let value: u32 = rlp.as_val()?;

            let lat = ((value & 0xffff0000) >> 16) as u16;
            let lng = (value & 0xffff) as u16;

            Ok(Coordinates {lat:lat, lng:lng})
        }
    }
}

impl fmt::Display for Coordinates {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(lat={}; lng={})",
            self.lat,
            self.lng,
        )
    }
}



/// Approximate earth radius: allow a precision up to 1mm
static R: f64 = 6371.0;

/// Maximum unsigned 16 bit integer
static U16_MAX: f64 = 65535.0;

/// Amplitude of lat value in radian
static DELTA_LAT: f64 = PI;

/// Amplitude of lng value in radian
static DELTA_LNG: f64 =  2.0 * PI;

impl Coordinates {
    /// Constructor
    pub fn new() -> Coordinates {
        Coordinates {lat:0, lng:0}
    }

    // Constructor
    pub fn from_raw(lat: u16, lng: u16) -> Coordinates {
        Coordinates {lat: lat, lng: lng}
    }
    
    /// Compute distance with Haversine formula between self and argument
    /// TODO : make the computation consensus-safe (through builtin ?)
    pub fn distance(&self, coord: &Coordinates) -> u16 {
        haversine_dist(
            self.lat as f64 * DELTA_LAT / U16_MAX - PI / 2.0,
            self.lng as f64 * DELTA_LNG / U16_MAX - PI,
            coord.lat as f64 * DELTA_LAT / U16_MAX - PI / 2.0,
            coord.lng as f64 * DELTA_LNG / U16_MAX - PI
        ).round() as u16
    } 
}


/// from https://rosettacode.org/wiki/Haversine_formula#Rust
pub fn haversine_dist(th1: f64, mut ph1: f64, th2: f64, ph2: f64) -> f64 {
    ph1 -= ph2;
    let dz: f64 = th1.sin() - th2.sin();
    let dx: f64 = ph1.cos() * th1.cos() - th2.cos();
    let dy: f64 = ph1.sin() * th1.cos();
    ((dx * dx + dy * dy + dz * dz).sqrt() / 2.0).asin() * 2.0 * R
}

#[cfg(test)]
mod tests {
    use super::Coordinates;
    use ethereum_types::H32;
    
    #[test]
    fn coordinates_conversion_with_h32() {
        let coord = Coordinates {lat: 1, lng: 2222};
        let coord_h32 = H32::from(coord.clone());
        assert_eq!(coord_h32, H32::from([0, 1, 8, 174])); // 2222 = 8*2^8+174
        let coord_bis = Coordinates::from(coord_h32);
        assert_eq!(coord, coord_bis); 
    }

    #[test]
    fn distance_between_nancy_and_grenoble() {
        let coord1 = Coordinates {lat: 1, lng: 1};
        let coord2 = Coordinates {lat: 1, lng: 1};
        assert_eq!(coord1.distance(&coord2), 400);
   }

}


/* pub struct Location {
    coordinates: Coordinates, // Coordinates or H32 ???
    valid_until: U256,
}

impl fmt::Display for Location {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "(lat={}; lng={}; validi_until={})",
			self.coordinates.lat,
			self.coordinates.lng,
			self.valid_until,
		)
	}
} */
