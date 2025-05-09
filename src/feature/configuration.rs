use std::fmt;

/// Representation of a configurational template. Most applications
/// will use only `TH1` (counterclockwise) and `TH2` (clockwise).
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Configuration {
    AL1,
    AL2,
    OH1,
    OH2,
    OH3,
    OH4,
    OH5,
    OH6,
    OH7,
    OH8,
    OH9,
    OH10,
    OH11,
    OH12,
    OH13,
    OH14,
    OH15,
    OH16,
    OH17,
    OH18,
    OH19,
    OH20,
    OH21,
    OH22,
    OH23,
    OH24,
    OH25,
    OH26,
    OH27,
    OH28,
    OH29,
    OH30,
    SP1,
    SP2,
    SP3,
    TB1,
    TB2,
    TB3,
    TB4,
    TB5,
    TB6,
    TB7,
    TB8,
    TB9,
    TB10,
    TB11,
    TB12,
    TB13,
    TB14,
    TB15,
    TB16,
    TB17,
    TB18,
    TB19,
    TB20,
    TH1,
    TH2,
    UnspecifiedTH,
    UnspecifiedAL,
    UnspecifiedTB,
    UnspecifiedOH,
    UnspecifiedSP,
}

impl fmt::Display for Configuration {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::TH1 | Self::AL1 => "@",
                Self::TH2 | Self::AL2 => "@@",
                Self::TB1 => "@TB1",
                Self::TB2 => "@TB2",
                Self::TB3 => "@TB3",
                Self::TB4 => "@TB4",
                Self::TB5 => "@TB5",
                Self::TB6 => "@TB6",
                Self::TB7 => "@TB7",
                Self::TB8 => "@TB8",
                Self::TB9 => "@TB9",
                Self::TB10 => "@TB10",
                Self::TB11 => "@TB11",
                Self::TB12 => "@TB12",
                Self::TB13 => "@TB13",
                Self::TB14 => "@TB14",
                Self::TB15 => "@TB15",
                Self::TB16 => "@TB16",
                Self::TB17 => "@TB17",
                Self::TB18 => "@TB18",
                Self::TB19 => "@TB19",
                Self::TB20 => "@TH20",
                Self::OH1 => "@OH1",
                Self::OH2 => "@OH2",
                Self::OH3 => "@OH3",
                Self::OH4 => "@OH4",
                Self::OH5 => "@OH5",
                Self::OH6 => "@OH6",
                Self::OH7 => "@OH7",
                Self::OH8 => "@OH8",
                Self::OH9 => "@OH9",
                Self::OH10 => "@OH10",
                Self::OH11 => "@OH11",
                Self::OH12 => "@OH12",
                Self::OH13 => "@OH13",
                Self::OH14 => "@OH14",
                Self::OH15 => "@OH15",
                Self::OH16 => "@OH16",
                Self::OH17 => "@OH17",
                Self::OH18 => "@OH18",
                Self::OH19 => "@OH19",
                Self::OH20 => "@OH20",
                Self::OH21 => "@OH21",
                Self::OH22 => "@OH22",
                Self::OH23 => "@OH23",
                Self::OH24 => "@OH24",
                Self::OH25 => "@OH25",
                Self::OH26 => "@OH26",
                Self::OH27 => "@OH27",
                Self::OH28 => "@OH28",
                Self::OH29 => "@OH29",
                Self::OH30 => "@OH30",
                Self::SP1 => "@SP1",
                Self::SP2 => "@SP2",
                Self::SP3 => "@SP3",
                Self::UnspecifiedTH => "@TH",
                Self::UnspecifiedAL => "@AL",
                Self::UnspecifiedTB => "@TB",
                Self::UnspecifiedOH => "@OH",
                Self::UnspecifiedSP => "@SP",
            }
        )
    }
}
