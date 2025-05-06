use logos::Lexer;

use super::{error::ReadError, missing_character::missing_character, token::Token};
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
pub fn read_configuration(lexer: &mut Lexer<Token>) -> Result<Option<Configuration>, ReadError> {
    if let Some(token) = lexer.next() {
        match token {
            Ok(Token::Ampersand) => {
                if let Some(token) = lexer.next() {
                    match token {
                        Ok(Token::Ampersand) => Ok(Some(Configuration::TH2)),
                        Ok(Token::AL) => Ok(Some(allene(lexer)?)),
                        Ok(Token::OH) => Ok(Some(octahedral(lexer)?)),
                        Ok(Token::SP) => Ok(Some(square_planar(lexer)?)),
                        Ok(Token::TB) => Ok(Some(trigonal_bipyramidal(lexer)?)),
                        Ok(Token::TH) => Ok(Some(tetrahedral(lexer)?)),
                        _ => Ok(Some(Configuration::TH1)),
                    }
                } else {
                    todo!("read_configuration")
                }
            }
            _ => Ok(None),
        }
    } else {
        return Err(ReadError::EndOfLine);
    }
}

fn tetrahedral(lexer: &mut Lexer<Token>) -> Result<Configuration, ReadError> {
    if let Some(token) = lexer.next() {
        match token {
            Ok(Token::Integer(number)) => match number {
                1 => Ok(Configuration::TH1),
                2 => Ok(Configuration::TH2),
                3..=9 => return Err(ReadError::Character(lexer.span().start)),
                _ => unreachable!("allene"),
            },
            _ => Ok(Configuration::UnspecifiedTH),
        }
    } else {
        return Err(ReadError::EndOfLine);
    }
}

fn allene(lexer: &mut Lexer<Token>) -> Result<Configuration, ReadError> {
    if let Some(token) = lexer.next() {
        match token {
            Ok(Token::Integer(number)) => match number {
                1 => Ok(Configuration::AL1),
                2 => Ok(Configuration::AL2),
                _ => unreachable!(),
            },
            _ => Ok(Configuration::UnspecifiedAL),
        }
    } else {
        return Err(ReadError::EndOfLine);
    }
}

fn square_planar(lexer: &mut Lexer<Token>) -> Result<Configuration, ReadError> {
    if let Some(token) = lexer.next() {
        match token {
            Ok(Token::Integer(number)) => match number {
                1 => Ok(Configuration::SP1),
                2 => Ok(Configuration::SP2),
                3 => Ok(Configuration::SP3),
                _ => Err(ReadError::Character(lexer.span().start)),
            },
            _ => Ok(Configuration::UnspecifiedSP),
        }
    } else {
        return Err(ReadError::EndOfLine);
    }
}

fn trigonal_bipyramidal(lexer: &mut Lexer<Token>) -> Result<Configuration, ReadError> {
    if let Some(token) = lexer.next() {
        match token {
            Ok(Token::Integer(number)) => match number {
                1 => {
                    if let Some(token) = lexer.next() {
                        match token {
                            Ok(Token::Integer(number)) => match number {
                                0 => Ok(Configuration::TB10),
                                1 => Ok(Configuration::TB11),
                                2 => Ok(Configuration::TB12),
                                3 => Ok(Configuration::TB13),
                                4 => Ok(Configuration::TB14),
                                5 => Ok(Configuration::TB15),
                                6 => Ok(Configuration::TB16),
                                7 => Ok(Configuration::TB17),
                                8 => Ok(Configuration::TB18),
                                9 => Ok(Configuration::TB19),
                                _ => unreachable!("in TB10-19"),
                            },
                            _ => return Err(ReadError::Character(lexer.span().start)),
                        }
                    } else {
                        return Err(ReadError::EndOfLine);
                    }
                }
                2 => {
                    if let Some(token) = lexer.next() {
                        match token {
                            Ok(Token::Integer(number)) => match number {
                                0 => Ok(Configuration::TB20),
                                _ => Ok(Configuration::TB2),
                            },
                            _ => return Err(ReadError::Character(lexer.span().start)),
                        }
                    } else {
                        return Err(ReadError::EndOfLine);
                    }
                }
                3 => Ok(Configuration::TB3),
                4 => Ok(Configuration::TB4),
                5 => Ok(Configuration::TB5),
                6 => Ok(Configuration::TB6),
                7 => Ok(Configuration::TB7),
                8 => Ok(Configuration::TB8),
                9 => Ok(Configuration::TB9),
                _ => Err(ReadError::Character(lexer.span().start)),
            },
            _ => return Err(ReadError::Character(lexer.span().start)),
        }
    } else {
        return Err(ReadError::EndOfLine);
    }
}

fn octahedral(lexer: &mut Lexer<Token>) -> Result<Configuration, ReadError> {
    if let Some(token) = lexer.next() {
        match token {
            Ok(Token::Integer(number)) => match number {
                1 => {
                    if let Some(token) = lexer.next() {
                        match token {
                            Ok(Token::Integer(number)) => match number {
                                0 => Ok(Configuration::OH10),
                                1 => Ok(Configuration::OH11),
                                2 => Ok(Configuration::OH12),
                                3 => Ok(Configuration::OH13),
                                4 => Ok(Configuration::OH14),
                                5 => Ok(Configuration::OH15),
                                6 => Ok(Configuration::OH16),
                                7 => Ok(Configuration::OH17),
                                8 => Ok(Configuration::OH18),
                                9 => Ok(Configuration::OH19),
                                _ => unreachable!("OH1X"),
                            },
                            _ => return Err(ReadError::Character(lexer.span().start)),
                        }
                    } else {
                        return Err(ReadError::EndOfLine);
                    }
                }
                2 => {
                    if let Some(token) = lexer.next() {
                        match token {
                            Ok(Token::Integer(number)) => match number {
                                0 => Ok(Configuration::OH20),
                                1 => Ok(Configuration::OH21),
                                2 => Ok(Configuration::OH22),
                                3 => Ok(Configuration::OH23),
                                4 => Ok(Configuration::OH24),
                                5 => Ok(Configuration::OH25),
                                6 => Ok(Configuration::OH26),
                                7 => Ok(Configuration::OH27),
                                8 => Ok(Configuration::OH28),
                                9 => Ok(Configuration::OH29),
                                _ => unreachable!("OH2X"),
                            },
                            _ => Err(ReadError::Character(lexer.span().start)),
                        }
                    } else {
                        return Err(ReadError::EndOfLine);
                    }
                }
                3 => {
                    if let Some(token) = lexer.next() {
                        match token {
                            Ok(Token::Integer(number)) => match number {
                                0 => Ok(Configuration::OH30),
                                _ => Ok(Configuration::OH3),
                            },
                            _ => unreachable!("octahedral - inner"),
                        }
                    } else {
                        return Err(ReadError::EndOfLine);
                    }
                }
                4 => Ok(Configuration::OH4),
                5 => Ok(Configuration::OH5),
                6 => Ok(Configuration::OH6),
                7 => Ok(Configuration::OH7),
                8 => Ok(Configuration::OH8),
                9 => Ok(Configuration::OH9),
                _ => Ok(Configuration::UnspecifiedOH),
            },
            _ => unreachable!("octahedral"),
        }
    } else {
        return Err(ReadError::EndOfLine);
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use logos::Logos;
    use pretty_assertions::assert_eq;

    #[test]
    fn unspecified_th() {
        let mut lexer = Token::lexer("@TH");

        assert_eq!(
            read_configuration(&mut lexer),
            Ok(Some(Configuration::UnspecifiedTH))
        )
    }

    #[test]
    fn unspecified_al() {
        let mut lexer = Token::lexer("@AL");

        assert_eq!(
            read_configuration(&mut lexer),
            Ok(Some(Configuration::UnspecifiedAL))
        )
    }

    #[test]
    fn unspecified_sp() {
        let mut lexer = Token::lexer("@SP");

        assert_eq!(
            read_configuration(&mut lexer),
            Ok(Some(Configuration::UnspecifiedSP))
        )
    }

    #[test]
    fn unspecified_tb() {
        let mut lexer = Token::lexer("@TB");

        assert_eq!(
            read_configuration(&mut lexer),
            Ok(Some(Configuration::UnspecifiedTB))
        )
    }

    #[test]
    fn unspecified_oh() {
        let mut lexer = Token::lexer("@OH");

        assert_eq!(
            read_configuration(&mut lexer),
            Ok(Some(Configuration::UnspecifiedOH))
        )
    }

    #[test]
    fn counterclockwise() {
        let mut lexer = Token::lexer("@");

        assert_eq!(read_configuration(&mut lexer), Ok(Some(Configuration::TH1)))
    }

    #[test]
    fn clockwise() {
        let mut lexer = Token::lexer("@@");

        assert_eq!(read_configuration(&mut lexer), Ok(Some(Configuration::TH2)))
    }

    #[test]
    fn th_1() {
        let mut lexer = Token::lexer("@TH1");

        assert_eq!(read_configuration(&mut lexer), Ok(Some(Configuration::TH1)))
    }

    #[test]
    fn th_2() {
        let mut lexer = Token::lexer("@TH2");

        assert_eq!(read_configuration(&mut lexer), Ok(Some(Configuration::TH2)))
    }

    #[test]
    fn th_unspecified() {
        let mut lexer = Token::lexer("@TH");

        assert_eq!(
            read_configuration(&mut lexer),
            Ok(Some(Configuration::UnspecifiedTH))
        )
    }

    #[test]
    fn al_1() {
        let mut lexer = Token::lexer("@AL1");

        assert_eq!(read_configuration(&mut lexer), Ok(Some(Configuration::AL1)))
    }

    #[test]
    fn al_2() {
        let mut lexer = Token::lexer("@AL2");

        assert_eq!(read_configuration(&mut lexer), Ok(Some(Configuration::AL2)))
    }

    #[test]
    fn tb_1() {
        let mut lexer = Token::lexer("@TB1");

        assert_eq!(read_configuration(&mut lexer), Ok(Some(Configuration::TB1)))
    }

    #[test]
    fn tb_2() {
        let mut lexer = Token::lexer("@TB2");

        assert_eq!(read_configuration(&mut lexer), Ok(Some(Configuration::TB2)))
    }

    #[test]
    fn tb_5() {
        let mut lexer = Token::lexer("@TB5");

        assert_eq!(read_configuration(&mut lexer), Ok(Some(Configuration::TB5)))
    }

    #[test]
    fn tb_7() {
        let mut lexer = Token::lexer("@TB7");

        assert_eq!(read_configuration(&mut lexer), Ok(Some(Configuration::TB7)))
    }

    #[test]
    fn tb_10() {
        let mut lexer = Token::lexer("@TB10");

        assert_eq!(
            read_configuration(&mut lexer),
            Ok(Some(Configuration::TB10))
        )
    }

    #[test]
    fn tb_19() {
        let mut lexer = Token::lexer("@TB19");

        assert_eq!(
            read_configuration(&mut lexer),
            Ok(Some(Configuration::TB19))
        )
    }

    #[test]
    fn tb_20() {
        let mut lexer = Token::lexer("@TB20");

        assert_eq!(
            read_configuration(&mut lexer),
            Ok(Some(Configuration::TB20))
        )
    }

    #[test]
    fn tb_unspecified() {
        let mut lexer = Token::lexer("@TB");

        assert_eq!(
            read_configuration(&mut lexer),
            Ok(Some(Configuration::UnspecifiedTB))
        )
    }

    #[test]
    fn oh_1() {
        let mut lexer = Token::lexer("@OH1");

        assert_eq!(read_configuration(&mut lexer), Ok(Some(Configuration::OH1)))
    }

    #[test]
    fn oh_2() {
        let mut lexer = Token::lexer("@OH2");

        assert_eq!(read_configuration(&mut lexer), Ok(Some(Configuration::OH2)))
    }

    #[test]
    fn oh_3() {
        let mut lexer = Token::lexer("@OH3");

        assert_eq!(read_configuration(&mut lexer), Ok(Some(Configuration::OH3)))
    }

    #[test]
    fn oh_5() {
        let mut lexer = Token::lexer("@OH5");

        assert_eq!(read_configuration(&mut lexer), Ok(Some(Configuration::OH5)))
    }

    #[test]
    fn oh_10() {
        let mut lexer = Token::lexer("@OH10");

        assert_eq!(
            read_configuration(&mut lexer),
            Ok(Some(Configuration::OH10))
        )
    }

    #[test]
    fn oh_15() {
        let mut lexer = Token::lexer("@OH15");

        assert_eq!(
            read_configuration(&mut lexer),
            Ok(Some(Configuration::OH15))
        )
    }

    #[test]
    fn oh_20() {
        let mut lexer = Token::lexer("@OH20");

        assert_eq!(
            read_configuration(&mut lexer),
            Ok(Some(Configuration::OH20))
        )
    }

    #[test]
    fn oh_25() {
        let mut lexer = Token::lexer("@OH25");

        assert_eq!(
            read_configuration(&mut lexer),
            Ok(Some(Configuration::OH25))
        )
    }

    #[test]
    fn oh_30() {
        let mut lexer = Token::lexer("@OH30");

        assert_eq!(
            read_configuration(&mut lexer),
            Ok(Some(Configuration::OH30))
        )
    }

    #[test]
    fn oh_unspecified() {
        let mut lexer = Token::lexer("@OH");

        assert_eq!(
            read_configuration(&mut lexer),
            Ok(Some(Configuration::UnspecifiedOH))
        )
    }

    #[test]
    fn sp_1() {
        let mut lexer = Token::lexer("@SP1");

        assert_eq!(read_configuration(&mut lexer), Ok(Some(Configuration::SP1)))
    }

    #[test]
    fn sp_2() {
        let mut lexer = Token::lexer("@SP2");

        assert_eq!(read_configuration(&mut lexer), Ok(Some(Configuration::SP2)))
    }

    #[test]
    fn sp_3() {
        let mut lexer = Token::lexer("@SP3");

        assert_eq!(read_configuration(&mut lexer), Ok(Some(Configuration::SP3)))
    }

    #[test]
    fn sp_unspecified() {
        let mut lexer = Token::lexer("@SP");

        assert_eq!(
            read_configuration(&mut lexer),
            Ok(Some(Configuration::UnspecifiedSP))
        )
    }
}
