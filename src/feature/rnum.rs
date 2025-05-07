use std::convert::TryFrom;
use std::fmt;

// A ring closure digit (rnum), as described in
/// [OpenSMILES](http://opensmiles.org/opensmiles.html).
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, PartialOrd, Ord)]
pub struct Rnum(u8);

impl Rnum {
    pub fn new(n: u8) -> Self {
        assert!(n <= 99, "Rnum must be in 0..=99");
        Self(n)
    }
}

impl TryFrom<u16> for Rnum {
    type Error = ();

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        if value <= 99 {
            Ok(Self(u8::try_from(value).expect("convert u16 to u8")))
        } else {
            Err(())
        }
    }
}

impl fmt::Display for Rnum {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.0 {
            0..=9 => write!(f, "{}", self.0),
            10..=99 => write!(f, "%{:02}", self.0),
            _ => unreachable!(),
        }
    }
}
