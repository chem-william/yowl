use std::convert::TryFrom;
use std::fmt;

/// An integer charge from -15 to +15.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Charge {
    MinusFifteen,
    MinusFourteen,
    MinusThirteen,
    MinusTwelve,
    MinusEleven,
    MinusTen,
    MinusNine,
    MinusEight,
    MinusSeven,
    MinusSix,
    MinusFive,
    MinusFour,
    MinusThree,
    MinusTwo,
    MinusOne,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Eleven,
    Twelve,
    Thirteen,
    Fourteen,
    Fifteen,
}

impl TryFrom<i8> for Charge {
    type Error = ();

    fn try_from(value: i8) -> Result<Self, Self::Error> {
        Ok(match value {
            -15 => Self::MinusFifteen,
            -14 => Self::MinusFourteen,
            -13 => Self::MinusThirteen,
            -12 => Self::MinusTwelve,
            -11 => Self::MinusEleven,
            -10 => Self::MinusTen,
            -9 => Self::MinusNine,
            -8 => Self::MinusEight,
            -7 => Self::MinusSeven,
            -6 => Self::MinusSix,
            -5 => Self::MinusFive,
            -4 => Self::MinusFour,
            -3 => Self::MinusThree,
            -2 => Self::MinusTwo,
            -1 => Self::MinusOne,
            1 => Self::One,
            2 => Self::Two,
            3 => Self::Three,
            4 => Self::Four,
            5 => Self::Five,
            6 => Self::Six,
            7 => Self::Seven,
            8 => Self::Eight,
            9 => Self::Nine,
            10 => Self::Ten,
            11 => Self::Eleven,
            12 => Self::Twelve,
            13 => Self::Thirteen,
            14 => Self::Fourteen,
            15 => Self::Fifteen,
            _ => return Err(()),
        })
    }
}

impl From<&Charge> for i8 {
    fn from(val: &Charge) -> Self {
        match val {
            Charge::MinusFifteen => -15,
            Charge::MinusFourteen => -14,
            Charge::MinusThirteen => -13,
            Charge::MinusTwelve => -12,
            Charge::MinusEleven => -11,
            Charge::MinusTen => -10,
            Charge::MinusNine => -9,
            Charge::MinusEight => -8,
            Charge::MinusSeven => -7,
            Charge::MinusSix => -6,
            Charge::MinusFive => -5,
            Charge::MinusFour => -4,
            Charge::MinusThree => -3,
            Charge::MinusTwo => -2,
            Charge::MinusOne => -1,
            Charge::One => 1,
            Charge::Two => 2,
            Charge::Three => 3,
            Charge::Four => 4,
            Charge::Five => 5,
            Charge::Six => 6,
            Charge::Seven => 7,
            Charge::Eight => 8,
            Charge::Nine => 9,
            Charge::Ten => 10,
            Charge::Eleven => 11,
            Charge::Twelve => 12,
            Charge::Thirteen => 13,
            Charge::Fourteen => 14,
            Charge::Fifteen => 15,
        }
    }
}

impl fmt::Display for Charge {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::MinusFifteen => "-15",
                Self::MinusFourteen => "-14",
                Self::MinusThirteen => "-13",
                Self::MinusTwelve => "-12",
                Self::MinusEleven => "-11",
                Self::MinusTen => "-10",
                Self::MinusNine => "-9",
                Self::MinusEight => "-8",
                Self::MinusSeven => "-7",
                Self::MinusSix => "-6",
                Self::MinusFive => "-5",
                Self::MinusFour => "-4",
                Self::MinusThree => "-3",
                Self::MinusTwo => "-2",
                Self::MinusOne => "-",
                Self::One => "+",
                Self::Two => "+2",
                Self::Three => "+3",
                Self::Four => "+4",
                Self::Five => "+5",
                Self::Six => "+6",
                Self::Seven => "+7",
                Self::Eight => "+8",
                Self::Nine => "+9",
                Self::Ten => "+10",
                Self::Eleven => "+11",
                Self::Twelve => "+12",
                Self::Thirteen => "+13",
                Self::Fourteen => "+14",
                Self::Fifteen => "+15",
            }
        )
    }
}
