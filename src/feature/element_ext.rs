use mendeleev::Element;

use super::atom_kind;

pub trait ElementExt {
    fn targets(&self) -> &[u8];
}

impl ElementExt for Element {
    fn targets(&self) -> &[u8] {
        atom_kind::elemental_targets(*self, None)
    }
}
