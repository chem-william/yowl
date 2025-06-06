use crate::feature::{AtomKind, BondKind, Rnum};
use crate::walk::Follower;

/// A `Follower` that builds a string SMILEs representation.
///
/// ```
/// use yowl::walk::Follower;
/// use yowl::write::Writer;
/// use yowl::feature::{AtomKind, BondKind, Symbol};
///
/// let mut writer = Writer::default();
///
/// writer.root(AtomKind::Symbol(Symbol::Star));
/// writer.extend(BondKind::Double, AtomKind::Symbol(Symbol::Star));
///
/// assert_eq!(writer.write(), "*=*")
/// ```
#[derive(Debug, PartialEq, Eq, Default)]
pub struct Writer {
    stack: Vec<String>,
}

impl Writer {
    pub fn write(self) -> String {
        self.stack.join("")
    }
}

impl Follower for Writer {
    fn root(&mut self, root: AtomKind) {
        if self.stack.is_empty() {
            self.stack.push(root.to_string());
        } else {
            self.stack.push(".".to_string() + &root.to_string());
        }
    }

    fn extend(&mut self, bond_kind: BondKind, atom_kind: AtomKind) {
        self.stack
            .push(bond_kind.to_string() + &atom_kind.to_string());
    }

    fn join(&mut self, bond_kind: BondKind, rnum: Rnum) {
        let last = self.stack.last_mut().expect("last");

        last.push_str(&(bond_kind.to_string() + &rnum.to_string()));
    }

    fn pop(&mut self, depth: usize) {
        assert!(depth < self.stack.len(), "overpop");

        let chain = self.stack.split_off(self.stack.len() - depth);
        let last = self.stack.last_mut().expect("last");

        last.push_str(&("(".to_string() + &chain.join("") + ")"));
    }
}

#[cfg(test)]
mod write {
    use crate::feature::Symbol;

    use super::*;
    use crate::Element;
    use pretty_assertions::assert_eq;

    #[test]
    fn p1() {
        let mut writer = Writer::default();

        writer.root(AtomKind::Symbol(Symbol::Star));

        assert_eq!(writer.write(), "*")
    }

    #[test]
    fn p2() {
        let mut writer = Writer::default();

        writer.root(AtomKind::Symbol(Symbol::Star));
        writer.extend(BondKind::Single, AtomKind::Symbol(Symbol::Star));

        assert_eq!(writer.write(), "*-*")
    }

    #[test]
    fn p1_p1() {
        let mut writer = Writer::default();

        writer.root(AtomKind::Symbol(Symbol::Star));
        writer.root(AtomKind::Symbol(Symbol::Star));

        assert_eq!(writer.write(), "*.*")
    }

    #[test]
    fn p3() {
        let mut writer = Writer::default();

        writer.root(AtomKind::Symbol(Symbol::Star));
        writer.extend(BondKind::Single, AtomKind::Symbol(Symbol::Star));
        writer.extend(BondKind::Single, AtomKind::Symbol(Symbol::Star));

        assert_eq!(writer.write(), "*-*-*")
    }

    #[test]
    fn p3_branched() {
        let mut writer = Writer::default();

        writer.root(AtomKind::Symbol(Symbol::Star));
        writer.extend(
            BondKind::Elided,
            AtomKind::Symbol(Symbol::Aliphatic(Element::F)),
        );
        writer.pop(1);
        writer.extend(
            BondKind::Elided,
            AtomKind::Symbol(Symbol::Aliphatic(Element::Cl)),
        );

        assert_eq!(writer.write(), "*(F)Cl")
    }

    #[test]
    fn c3() {
        let mut writer = Writer::default();

        writer.root(AtomKind::Symbol(Symbol::Star));
        writer.join(BondKind::Single, Rnum::new(1));
        writer.extend(BondKind::Single, AtomKind::Symbol(Symbol::Star));
        writer.extend(BondKind::Double, AtomKind::Symbol(Symbol::Star));
        writer.join(BondKind::Single, Rnum::new(1));

        assert_eq!(writer.write(), "*-1-*=*-1")
    }

    #[test]
    fn c3_branched() {
        let mut writer = Writer::default();

        writer.root(AtomKind::Symbol(Symbol::Star));
        writer.extend(BondKind::Elided, AtomKind::Symbol(Symbol::Star));
        writer.extend(BondKind::Elided, AtomKind::Symbol(Symbol::Star));
        writer.join(BondKind::Elided, Rnum::new(1));
        writer.pop(2);
        writer.join(BondKind::Elided, Rnum::new(1));

        assert_eq!(writer.write(), "*(**1)1")
    }

    #[test]
    fn nested_branch() {
        let mut writer = Writer::default();

        writer.root(AtomKind::Symbol(Symbol::Star));
        writer.extend(BondKind::Elided, AtomKind::Symbol(Symbol::Star));
        writer.extend(BondKind::Single, AtomKind::Symbol(Symbol::Star));
        writer.pop(1);
        writer.extend(BondKind::Elided, AtomKind::Symbol(Symbol::Star));
        writer.pop(2);
        writer.extend(BondKind::Double, AtomKind::Symbol(Symbol::Star));

        assert_eq!(writer.write(), "*(*(-*)*)=*")
    }
}
