use super::{error::ReadError, missing_character::missing_character, scanner::Scanner};
use crate::feature::Configuration;

/// Reads the configuration of a molecule from the scanner.
/// The configuration is specified using the following syntax:
/// - `@` for counterclockwise tetrahedral chirality
/// - `@@` for clockwise tetrahedral chirality
/// - `@TH1` for counterclockwise tetrahedral chirality (same as `@`)
/// - `@TH2` for clockwise tetrahedral chirality (same as `@@`)
/// - `@AL1` for allene configuration 1 (same as `@`)
/// - `@AL2` for allene configuration 2 (same as `@@`)
///
/// If only the configuration is specified (whether it's TH, AL, etc.), but not the specific chirality (@TH1, @AL2, etc.)
/// then UnspecifiedXX is returned where `XX` specifies the configuration.
pub fn read_configuration(scanner: &mut Scanner) -> Result<Option<Configuration>, ReadError> {
    Ok(Some(match scanner.peek() {
        Some('@') => {
            scanner.pop();

            match scanner.peek() {
                Some('@') => {
                    scanner.pop();

                    Configuration::TH2
                }
                Some('A') => {
                    scanner.pop();

                    match scanner.peek() {
                        Some('L') => {
                            scanner.pop();

                            allene(scanner)?
                        }
                        _ => unreachable!("Should've hit UnspecifiedAL"),
                    }
                }
                Some('O') => {
                    scanner.pop();

                    match scanner.peek() {
                        Some('H') => {
                            scanner.pop();

                            octahedral(scanner)?
                        }
                        _ => unreachable!("Should've hit UnspecifiedOH"),
                    }
                }
                Some('S') => {
                    scanner.pop();

                    match scanner.peek() {
                        Some('P') => {
                            scanner.pop();

                            square_planar(scanner)?
                        }
                        _ => unreachable!("Should've hit UnspecifiedSP"),
                    }
                }
                Some('T') => {
                    scanner.pop();

                    match scanner.peek() {
                        Some('B') => {
                            scanner.pop();

                            trigonal_bipyramidal(scanner)?
                        }
                        Some('H') => {
                            scanner.pop();

                            tetrahedral(scanner)?
                        }
                        _ => unreachable!("Should've hit UnspecifiedTB or TH"),
                    }
                }
                _ => Configuration::TH1,
            }
        }
        _ => return Ok(None),
    }))
}

fn tetrahedral(scanner: &mut Scanner) -> Result<Configuration, ReadError> {
    Ok(match scanner.peek() {
        Some('1') => {
            scanner.pop();

            Configuration::TH1
        }
        Some('2') => {
            scanner.pop();

            Configuration::TH2
        }
        Some('3'..='9') => return Err(missing_character(scanner)),
        _ => Configuration::UnspecifiedTH, // Stereochemistry not specified
    })
}

fn allene(scanner: &mut Scanner) -> Result<Configuration, ReadError> {
    Ok(match scanner.peek() {
        Some('1') => {
            scanner.pop();

            Configuration::AL1
        }
        Some('2') => {
            scanner.pop();

            Configuration::AL2
        }
        _ => Configuration::UnspecifiedAL,
    })
}

fn square_planar(scanner: &mut Scanner) -> Result<Configuration, ReadError> {
    Ok(match scanner.peek() {
        Some('1') => {
            scanner.pop();

            Configuration::SP1
        }
        Some('2') => {
            scanner.pop();

            Configuration::SP2
        }
        Some('3') => {
            scanner.pop();

            Configuration::SP3
        }
        _ => Configuration::UnspecifiedSP, // Stereochemistry not specified
    })
}

fn trigonal_bipyramidal(scanner: &mut Scanner) -> Result<Configuration, ReadError> {
    Ok(match scanner.peek() {
        Some('1') => {
            scanner.pop();

            match scanner.peek() {
                Some('0'..='9') => match scanner.pop() {
                    Some('0') => Configuration::TB10,
                    Some('1') => Configuration::TB11,
                    Some('2') => Configuration::TB12,
                    Some('3') => Configuration::TB13,
                    Some('4') => Configuration::TB14,
                    Some('5') => Configuration::TB15,
                    Some('6') => Configuration::TB16,
                    Some('7') => Configuration::TB17,
                    Some('8') => Configuration::TB18,
                    Some('9') => Configuration::TB19,
                    _ => unreachable!("TB1X"),
                },
                _ => Configuration::TB1,
            }
        }
        Some('2') => {
            scanner.pop();

            match scanner.peek() {
                Some('0') => {
                    scanner.pop();

                    Configuration::TB20
                }
                _ => Configuration::TB2,
            }
        }
        Some('3') => {
            scanner.pop();

            Configuration::TB3
        }
        Some('4') => {
            scanner.pop();
            Configuration::TB4
        }
        Some('5') => {
            scanner.pop();
            Configuration::TB5
        }
        Some('6') => {
            scanner.pop();
            Configuration::TB6
        }
        Some('7') => {
            scanner.pop();
            Configuration::TB7
        }
        Some('8') => {
            scanner.pop();
            Configuration::TB8
        }
        Some('9') => {
            scanner.pop();
            Configuration::TB9
        }
        _ => Configuration::UnspecifiedTB, // Stereochemistry not specified
    })
}

fn octahedral(scanner: &mut Scanner) -> Result<Configuration, ReadError> {
    Ok(match scanner.peek() {
        Some('1') => {
            scanner.pop();

            match scanner.peek() {
                Some('0'..='9') => match scanner.pop() {
                    Some('0') => Configuration::OH10,
                    Some('1') => Configuration::OH11,
                    Some('2') => Configuration::OH12,
                    Some('3') => Configuration::OH13,
                    Some('4') => Configuration::OH14,
                    Some('5') => Configuration::OH15,
                    Some('6') => Configuration::OH16,
                    Some('7') => Configuration::OH17,
                    Some('8') => Configuration::OH18,
                    Some('9') => Configuration::OH19,
                    _ => unreachable!("OH1X"),
                },
                _ => Configuration::OH1,
            }
        }
        Some('2') => {
            scanner.pop();

            match scanner.peek() {
                Some('0'..='9') => match scanner.pop() {
                    Some('0') => Configuration::OH20,
                    Some('1') => Configuration::OH21,
                    Some('2') => Configuration::OH22,
                    Some('3') => Configuration::OH23,
                    Some('4') => Configuration::OH24,
                    Some('5') => Configuration::OH25,
                    Some('6') => Configuration::OH26,
                    Some('7') => Configuration::OH27,
                    Some('8') => Configuration::OH28,
                    Some('9') => Configuration::OH29,
                    _ => unreachable!("OH2X"),
                },
                _ => Configuration::OH2,
            }
        }
        Some('3') => {
            scanner.pop();

            match scanner.peek() {
                Some('0') => {
                    scanner.pop();

                    Configuration::OH30
                }
                _ => Configuration::OH3,
            }
        }
        Some('4') => {
            scanner.pop();
            Configuration::OH4
        }
        Some('5') => {
            scanner.pop();
            Configuration::OH5
        }
        Some('6') => {
            scanner.pop();
            Configuration::OH6
        }
        Some('7') => {
            scanner.pop();
            Configuration::OH7
        }
        Some('8') => {
            scanner.pop();
            Configuration::OH8
        }
        Some('9') => {
            scanner.pop();
            Configuration::OH9
        }
        _ => Configuration::UnspecifiedOH, // Stereochemistry not specified
    })
}
#[test]
fn unspecified_th() {
    let mut scanner = Scanner::new("@TH");

    assert_eq!(
        read_configuration(&mut scanner),
        Ok(Some(Configuration::UnspecifiedTH))
    )
}

#[test]
fn unspecified_al() {
    let mut scanner = Scanner::new("@AL");

    assert_eq!(
        read_configuration(&mut scanner),
        Ok(Some(Configuration::UnspecifiedAL))
    )
}

#[test]
fn unspecified_sp() {
    let mut scanner = Scanner::new("@SP");

    assert_eq!(
        read_configuration(&mut scanner),
        Ok(Some(Configuration::UnspecifiedSP))
    )
}

#[test]
fn unspecified_tb() {
    let mut scanner = Scanner::new("@TB");

    assert_eq!(
        read_configuration(&mut scanner),
        Ok(Some(Configuration::UnspecifiedTB))
    )
}

#[test]
fn unspecified_oh() {
    let mut scanner = Scanner::new("@OH");

    assert_eq!(
        read_configuration(&mut scanner),
        Ok(Some(Configuration::UnspecifiedOH))
    )
}
#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn counterclockwise() {
        let mut scanner = Scanner::new("@");

        assert_eq!(
            read_configuration(&mut scanner),
            Ok(Some(Configuration::TH1))
        )
    }

    #[test]
    fn clockwise() {
        let mut scanner = Scanner::new("@@");

        assert_eq!(
            read_configuration(&mut scanner),
            Ok(Some(Configuration::TH2))
        )
    }

    #[test]
    fn th_1() {
        let mut scanner = Scanner::new("@TH1");

        assert_eq!(
            read_configuration(&mut scanner),
            Ok(Some(Configuration::TH1))
        )
    }

    #[test]
    fn th_2() {
        let mut scanner = Scanner::new("@TH2");

        assert_eq!(
            read_configuration(&mut scanner),
            Ok(Some(Configuration::TH2))
        )
    }

    #[test]
    fn th_unspecified() {
        let mut scanner = Scanner::new("@TH");

        assert_eq!(
            read_configuration(&mut scanner),
            Ok(Some(Configuration::UnspecifiedTH))
        )
    }

    #[test]
    fn al_1() {
        let mut scanner = Scanner::new("@AL1");

        assert_eq!(
            read_configuration(&mut scanner),
            Ok(Some(Configuration::AL1))
        )
    }

    #[test]
    fn al_2() {
        let mut scanner = Scanner::new("@AL2");

        assert_eq!(
            read_configuration(&mut scanner),
            Ok(Some(Configuration::AL2))
        )
    }

    #[test]
    fn tb_1() {
        let mut scanner = Scanner::new("@TB1");

        assert_eq!(
            read_configuration(&mut scanner),
            Ok(Some(Configuration::TB1))
        )
    }

    #[test]
    fn tb_2() {
        let mut scanner = Scanner::new("@TB2");

        assert_eq!(
            read_configuration(&mut scanner),
            Ok(Some(Configuration::TB2))
        )
    }

    #[test]
    fn tb_5() {
        let mut scanner = Scanner::new("@TB5");

        assert_eq!(
            read_configuration(&mut scanner),
            Ok(Some(Configuration::TB5))
        )
    }

    #[test]
    fn tb_7() {
        let mut scanner = Scanner::new("@TB7");

        assert_eq!(
            read_configuration(&mut scanner),
            Ok(Some(Configuration::TB7))
        )
    }

    #[test]
    fn tb_10() {
        let mut scanner = Scanner::new("@TB10");

        assert_eq!(
            read_configuration(&mut scanner),
            Ok(Some(Configuration::TB10))
        )
    }

    #[test]
    fn tb_19() {
        let mut scanner = Scanner::new("@TB19");

        assert_eq!(
            read_configuration(&mut scanner),
            Ok(Some(Configuration::TB19))
        )
    }

    #[test]
    fn tb_20() {
        let mut scanner = Scanner::new("@TB20");

        assert_eq!(
            read_configuration(&mut scanner),
            Ok(Some(Configuration::TB20))
        )
    }

    #[test]
    fn tb_unspecified() {
        let mut scanner = Scanner::new("@TB");

        assert_eq!(
            read_configuration(&mut scanner),
            Ok(Some(Configuration::UnspecifiedTB))
        )
    }

    #[test]
    fn oh_1() {
        let mut scanner = Scanner::new("@OH1");

        assert_eq!(
            read_configuration(&mut scanner),
            Ok(Some(Configuration::OH1))
        )
    }

    #[test]
    fn oh_2() {
        let mut scanner = Scanner::new("@OH2");

        assert_eq!(
            read_configuration(&mut scanner),
            Ok(Some(Configuration::OH2))
        )
    }

    #[test]
    fn oh_3() {
        let mut scanner = Scanner::new("@OH3");

        assert_eq!(
            read_configuration(&mut scanner),
            Ok(Some(Configuration::OH3))
        )
    }

    #[test]
    fn oh_5() {
        let mut scanner = Scanner::new("@OH5");

        assert_eq!(
            read_configuration(&mut scanner),
            Ok(Some(Configuration::OH5))
        )
    }

    #[test]
    fn oh_10() {
        let mut scanner = Scanner::new("@OH10");

        assert_eq!(
            read_configuration(&mut scanner),
            Ok(Some(Configuration::OH10))
        )
    }

    #[test]
    fn oh_15() {
        let mut scanner = Scanner::new("@OH15");

        assert_eq!(
            read_configuration(&mut scanner),
            Ok(Some(Configuration::OH15))
        )
    }

    #[test]
    fn oh_20() {
        let mut scanner = Scanner::new("@OH20");

        assert_eq!(
            read_configuration(&mut scanner),
            Ok(Some(Configuration::OH20))
        )
    }

    #[test]
    fn oh_25() {
        let mut scanner = Scanner::new("@OH25");

        assert_eq!(
            read_configuration(&mut scanner),
            Ok(Some(Configuration::OH25))
        )
    }

    #[test]
    fn oh_30() {
        let mut scanner = Scanner::new("@OH30");

        assert_eq!(
            read_configuration(&mut scanner),
            Ok(Some(Configuration::OH30))
        )
    }

    #[test]
    fn oh_unspecified() {
        let mut scanner = Scanner::new("@OH");

        assert_eq!(
            read_configuration(&mut scanner),
            Ok(Some(Configuration::UnspecifiedOH))
        )
    }

    #[test]
    fn sp_1() {
        let mut scanner = Scanner::new("@SP1");

        assert_eq!(
            read_configuration(&mut scanner),
            Ok(Some(Configuration::SP1))
        )
    }

    #[test]
    fn sp_2() {
        let mut scanner = Scanner::new("@SP2");

        assert_eq!(
            read_configuration(&mut scanner),
            Ok(Some(Configuration::SP2))
        )
    }

    #[test]
    fn sp_3() {
        let mut scanner = Scanner::new("@SP3");

        assert_eq!(
            read_configuration(&mut scanner),
            Ok(Some(Configuration::SP3))
        )
    }

    #[test]
    fn sp_unspecified() {
        let mut scanner = Scanner::new("@SP");

        assert_eq!(
            read_configuration(&mut scanner),
            Ok(Some(Configuration::UnspecifiedSP))
        )
    }
}
