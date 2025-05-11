use core::fmt;

/// A formal atomic charge between -15 and +15, inclusive.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Charge {
    pub(crate) value: i8,
}

impl Charge {
    /// The inclusive range of valid charges.
    pub const MIN: i8 = -15;
    pub const MAX: i8 = 15;

    /// Tries to construct a `Charge`, returning `None` if out of range.
    pub fn new(value: i8) -> Option<Self> {
        if (Self::MIN..=Self::MAX).contains(&value) {
            Some(Charge { value })
        } else {
            None
        }
    }

    /// Get the underlying `i8` back.
    pub fn value(self) -> i8 {
        self.value
    }
}

impl TryFrom<i8> for Charge {
    type Error = &'static str;

    fn try_from(value: i8) -> Result<Self, Self::Error> {
        Charge::new(value).ok_or("Charge out of range (-15…+15)")
    }
}

impl From<Charge> for i8 {
    fn from(c: Charge) -> i8 {
        c.value
    }
}

impl fmt::Display for Charge {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Print “+” for positive, “-” for negative, with no extra signs for ±1
        match self.value {
            0 => write!(f, "0"),
            n if n > 0 => {
                if n == 1 { write!(f, "+") }
                else { write!(f, "+{n}") }
            }
            n /* negative */ => {
                if n == -1 { write!(f, "-") }
                else { write!(f, "{n}") }
            }
        }
    }
}
