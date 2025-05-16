use std::fmt;

use mendeleev::{Element, Isotope};

use super::{Charge, Configuration, VirtualHydrogen};
use crate::feature::element_ext::ElementExt;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Symbol {
    Star,
    Aliphatic(Element),
    Aromatic(Element),
}

/// Minimal context-sensitive representation of an atom kind.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum AtomKind {
    Symbol(Symbol),
    Bracket {
        isotope: Option<Isotope>,
        symbol: Symbol,
        configuration: Option<Configuration>,
        hcount: Option<VirtualHydrogen>,
        charge: Option<Charge>,
        map: Option<u16>,
    },
}

impl AtomKind {
    /// Returns true if the kind was defined as being aromatic.
    pub const fn is_aromatic(&self) -> bool {
        match self {
            Self::Symbol(Symbol::Aromatic(_)) => true,
            Self::Symbol(Symbol::Aliphatic(_)) | Self::Symbol(Symbol::Star) => false,
            Self::Bracket { symbol, .. } => match symbol {
                Symbol::Aromatic(_) => true,
                Symbol::Aliphatic(_) | Symbol::Star => false,
            },
        }
    }

    /// Returns the valence targets for this atom kind.
    pub fn targets(&self) -> &[u8] {
        match self {
            Self::Symbol(Symbol::Star) => &[],
            Self::Symbol(Symbol::Aliphatic(aliphatic)) => aliphatic.targets(),
            Self::Symbol(Symbol::Aromatic(aromatic)) => aromatic.targets(),
            Self::Bracket { symbol, charge, .. } => match symbol {
                Symbol::Star => &[],
                Symbol::Aromatic(element) => elemental_targets(*element, *charge),
                Symbol::Aliphatic(element) => elemental_targets(*element, *charge),
            },
        }
    }

    /// Inverts configuration given if it and at least one implicit
    /// hydrogen are present.
    ///
    /// # Panics
    ///
    /// Panics given a Configuration other than TH1 or TH2.
    pub fn invert_configuration(&mut self) {
        if let Self::Bracket {
            hcount,
            configuration,
            ..
        } = self
        {
            let new_config = match configuration {
                Some(config) => match hcount {
                    Some(hcount) => {
                        if hcount.is_zero() {
                            return;
                        }
                        match config {
                            Configuration::TH1 => Configuration::TH2,
                            Configuration::TH2 => Configuration::TH1,
                            _ => unimplemented!("TODO: handle inversion for non-TH"),
                        }
                    }
                    None => return,
                },
                None => return,
            };

            configuration.replace(new_config);
        }
    }
}

pub const fn elemental_targets(element: Element, charge: Option<Charge>) -> &'static [u8] {
    match element {
        Element::B => match charge {
            Some(Charge { value: -3 }) => &OXYGEN_TARGET,
            Some(Charge { value: -2 }) => &NITROGEN_TARGET,
            Some(Charge { value: -1 }) => &CARBON_TARGET,
            None => &BORON_TARGET,
            _ => &EMPTY_TARGET,
        },
        Element::C | Element::Si => match charge {
            Some(Charge { value: -2 }) => &OXYGEN_TARGET,
            Some(Charge { value: -1 }) => &NITROGEN_TARGET,
            Some(Charge { value: 1 }) => &BORON_TARGET,
            None => &CARBON_TARGET,
            _ => &EMPTY_TARGET,
        },
        Element::N | Element::P | Element::As => match charge {
            Some(Charge { value: 1 }) => &CARBON_TARGET,
            Some(Charge { value: -1 }) => &SULFUR_TARGET,
            None => &NITROGEN_TARGET,
            _ => &EMPTY_TARGET,
        },
        Element::O => match charge {
            Some(Charge { value: 1 }) => &NITROGEN_TARGET,
            None => &OXYGEN_TARGET,
            _ => &EMPTY_TARGET,
        },
        Element::S | Element::Se | Element::Te => match charge {
            Some(Charge { value: 1 }) => &NITROGEN_TARGET,
            None => &SULFUR_TARGET,
            _ => &EMPTY_TARGET,
        },
        Element::F | Element::Cl | Element::Br | Element::I | Element::At | Element::Ts => {
            match charge {
                None => &HALOGEN_TARGET,
                _ => &EMPTY_TARGET,
            }
        }
        _ => &EMPTY_TARGET,
    }
}

const BORON_TARGET: [u8; 1] = [3];
const HALOGEN_TARGET: [u8; 1] = [1];
const CARBON_TARGET: [u8; 1] = [4];
const NITROGEN_TARGET: [u8; 2] = [3, 5];
const OXYGEN_TARGET: [u8; 1] = [2];
const SULFUR_TARGET: [u8; 3] = [2, 4, 6];
const EMPTY_TARGET: [u8; 0] = [];

impl fmt::Display for AtomKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Symbol(Symbol::Star) => write!(f, "*"),
            Self::Symbol(Symbol::Aliphatic(element)) => write!(f, "{}", element.symbol()),
            Self::Symbol(Symbol::Aromatic(element)) => {
                write!(f, "{}", element.symbol().to_lowercase())
            }
            Self::Bracket {
                isotope,
                symbol,
                hcount,
                configuration,
                charge,
                map,
            } => {
                write!(f, "[")?;

                if let Some(isotope) = isotope {
                    write!(f, "{}", isotope.mass_number())?;
                }

                match symbol {
                    Symbol::Star => write!(f, "*")?,
                    Symbol::Aliphatic(element) => write!(f, "{}", element.symbol())?,
                    Symbol::Aromatic(element) => write!(f, "{}", element.symbol().to_lowercase())?,
                }

                if let Some(configuration) = configuration {
                    write!(f, "{configuration}")?;
                }

                if let Some(hcount) = hcount {
                    write!(f, "{hcount}")?;
                }

                if let Some(charge) = charge {
                    write!(f, "{charge}")?;
                }

                if let Some(map) = map {
                    write!(f, ":{map}")?;
                }

                write!(f, "]")
            }
        }
    }
}

#[cfg(test)]
mod invert {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn aliphatic_organic() {
        let mut kind = AtomKind::Symbol(Symbol::Aliphatic(Element::C));

        kind.invert_configuration();

        match kind {
            AtomKind::Symbol(Symbol::Aliphatic(_)) => (),
            _ => panic!("expected aliphatic"),
        }
    }

    #[test]
    fn th1_h_none() {
        let mut kind = AtomKind::Bracket {
            isotope: None,
            symbol: Symbol::Star,
            configuration: Some(Configuration::TH1),
            hcount: None,
            charge: None,
            map: None,
        };

        kind.invert_configuration();

        match kind {
            AtomKind::Bracket { configuration, .. } => {
                assert_eq!(configuration, Some(Configuration::TH1))
            }
            _ => panic!("expected bracket"),
        }
    }

    #[test]
    fn th1_h1() {
        let mut kind = AtomKind::Bracket {
            isotope: None,
            symbol: Symbol::Star,
            configuration: Some(Configuration::TH1),
            hcount: Some(VirtualHydrogen::H1),
            charge: None,
            map: None,
        };

        kind.invert_configuration();

        match kind {
            AtomKind::Bracket { configuration, .. } => {
                assert_eq!(configuration, Some(Configuration::TH2))
            }
            _ => panic!("expected bracket"),
        }
    }

    #[test]
    fn th2_h1() {
        let mut kind = AtomKind::Bracket {
            isotope: None,
            symbol: Symbol::Star,
            configuration: Some(Configuration::TH2),
            hcount: Some(VirtualHydrogen::H1),
            charge: None,
            map: None,
        };

        kind.invert_configuration();

        match kind {
            AtomKind::Bracket { configuration, .. } => {
                assert_eq!(configuration, Some(Configuration::TH1))
            }
            _ => panic!("expected bracket"),
        }
    }

    #[test]
    fn is_aromatic_unbracketed() {
        assert!(!AtomKind::Symbol(Symbol::Star).is_aromatic());
        assert!(!AtomKind::Symbol(Symbol::Aliphatic(Element::N)).is_aromatic());
        assert!(AtomKind::Symbol(Symbol::Aromatic(Element::N)).is_aromatic());
    }

    #[test]
    fn display_simple_kinds() {
        assert_eq!(AtomKind::Symbol(Symbol::Star).to_string(), "*");
        assert_eq!(
            AtomKind::Symbol(Symbol::Aliphatic(Element::Br)).to_string(),
            "Br"
        );
        assert_eq!(
            AtomKind::Symbol(Symbol::Aromatic(Element::S)).to_string(),
            "s"
        );
    }

    #[test]
    fn targets_star_and_alph_and_arom() {
        let empty: &[u8] = &[];
        assert_eq!(AtomKind::Symbol(Symbol::Star).targets(), empty);
        assert_eq!(
            AtomKind::Symbol(Symbol::Aliphatic(Element::S)).targets(),
            &[2, 4, 6]
        );
        assert_eq!(
            AtomKind::Symbol(Symbol::Aromatic(Element::P)).targets(),
            &[3, 5]
        );
    }
}
