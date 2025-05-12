use std::fmt;

use mendeleev::{Element, Isotope};

use super::{BracketSymbol, Charge, Configuration, VirtualHydrogen};
use crate::feature::element_ext::ElementExt;

/// Minimal context-sensitive representation of an atom kind.
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum AtomKind {
    Star,
    Aliphatic(Element),
    Aromatic(Element),
    Bracket {
        isotope: Option<Isotope>,
        symbol: BracketSymbol,
        configuration: Option<Configuration>,
        hcount: Option<VirtualHydrogen>,
        charge: Option<Charge>,
        map: Option<u16>,
    },
}

impl AtomKind {
    /// Returns an unbracketed version of this `AtomKind` based on
    /// `bond_order_sum`, if possible. Already unbracketed `AtomKind`s return
    /// self.
    ///
    /// This method is intended for clients building representations from
    /// outside sources. It allows for a single, always valid bracketed `AtomKind`
    /// to be constructed and debracketed, if possible. The logic to decide
    /// debracketability is encapsulated here.
    pub fn debracket(self, bond_order_sum: u8) -> Self {
        let (isotope, symbol, configuration, hcount, charge, map) = match &self {
            Self::Star | Self::Aliphatic(_) | Self::Aromatic(_) => return self,
            Self::Bracket {
                isotope,
                symbol,
                configuration,
                hcount,
                charge,
                map,
            } => (isotope, symbol, configuration, hcount, charge, map),
        };

        if any(*isotope, *configuration, *charge, *map) {
            return self;
        }

        match symbol {
            BracketSymbol::Star => {
                hcount.as_ref().map_or(
                    Self::Star,
                    |hcount| if hcount.is_zero() { Self::Star } else { self },
                )
            }
            BracketSymbol::Aromatic(element) => {
                let hcount = hcount.as_ref().map_or(0, std::convert::Into::into);
                let valence = bond_order_sum.checked_add(hcount).expect("valence");
                let allowance = u8::from(hcount != 0);

                for target in element.targets() {
                    if valence == target - allowance {
                        return Self::Aromatic(*element);
                    }
                }

                self
            }
            BracketSymbol::Element(element) => {
                let valence = bond_order_sum
                    .checked_add(hcount.as_ref().map_or(0, std::convert::Into::into))
                    .expect("valence");

                for target in element.targets() {
                    if target == &valence {
                        return Self::Aliphatic(*element);
                    }
                }

                self
            }
        }
    }

    /// Returns true if the kind was defined as being aromatic.
    pub const fn is_aromatic(&self) -> bool {
        match self {
            Self::Aromatic(_) => true,
            Self::Aliphatic(_) | Self::Star => false,
            Self::Bracket { symbol, .. } => match symbol {
                BracketSymbol::Aromatic(_) => true,
                BracketSymbol::Element(_) | BracketSymbol::Star => false,
            },
        }
    }

    /// Returns the valence targets for this atom kind.
    pub fn targets(&self) -> &[u8] {
        match self {
            Self::Star => &[],
            Self::Aliphatic(aliphatic) => aliphatic.targets(),
            Self::Aromatic(aromatic) => aromatic.targets(),
            Self::Bracket { symbol, charge, .. } => match symbol {
                BracketSymbol::Star => &[],
                BracketSymbol::Aromatic(element) => elemental_targets(*element, *charge),
                BracketSymbol::Element(element) => elemental_targets(*element, *charge),
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

const fn any(
    isotope: Option<Isotope>,
    configuration: Option<Configuration>,
    charge: Option<Charge>,
    map: Option<u16>,
) -> bool {
    isotope.is_some() || configuration.is_some() || charge.is_some() || map.is_some()
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
            Self::Star => write!(f, "*"),
            Self::Aliphatic(element) => write!(f, "{}", element.symbol()),
            Self::Aromatic(element) => write!(f, "{}", element.symbol().to_lowercase()),
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

                write!(f, "{symbol}")?;

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
        let mut kind = AtomKind::Aliphatic(Element::C);

        kind.invert_configuration();

        match kind {
            AtomKind::Aliphatic(_) => (),
            _ => panic!("expected aliphatic"),
        }
    }

    #[test]
    fn th1_h_none() {
        let mut kind = AtomKind::Bracket {
            isotope: None,
            symbol: BracketSymbol::Star,
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
            symbol: BracketSymbol::Star,
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
            symbol: BracketSymbol::Star,
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
        assert!(!AtomKind::Star.is_aromatic());
        assert!(!AtomKind::Aliphatic(Element::N).is_aromatic());
        assert!(AtomKind::Aromatic(Element::N).is_aromatic());
    }

    #[test]
    fn display_simple_kinds() {
        assert_eq!(AtomKind::Star.to_string(), "*");
        assert_eq!(AtomKind::Aliphatic(Element::Br).to_string(), "Br");
        assert_eq!(AtomKind::Aromatic(Element::S).to_string(), "s");
    }

    #[test]
    fn targets_star_and_alph_and_arom() {
        let empty: &[u8] = &[];
        assert_eq!(AtomKind::Star.targets(), empty);
        assert_eq!(AtomKind::Aliphatic(Element::S).targets(), &[2, 4, 6]);
        assert_eq!(AtomKind::Aromatic(Element::P).targets(), &[3, 5]);
    }
}
