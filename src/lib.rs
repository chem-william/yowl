//! Primitives for reading and writing the [Simplified Molecular Line Input Line Entry System](https://en.wikipedia.org/wiki/Simplified_molecular-input_line-entry_system) (SMILES) language. Based on [OpenSMILES](http://opensmiles.org).
//! For goals and rationale, see:
//!
//! - [SMILES Formal Grammar](https://depth-first.com/articles/2020/05/25/lets-build-a-smiles-parser-in-rust/)
//! - [SMILES Formal Grammar Revisited](https://depth-first.com/articles/2020/04/20/smiles-formal-grammar/)
//! - [Let's Build a SMILES Parser in Rust](https://depth-first.com/articles/2020/12/14/an-abstract-syntatx-tree-for-smiles/)
//! - [Abstract Syntax Trees for SMILES](https://depth-first.com/articles/2020/12/21/smiles-formal-grammar-revisited/)

/// Common components used in `graph` and `tree` representations.
pub mod feature;
/// SMILES adjacency list representation.
pub mod graph;
/// Reading SMILES representations from strings.
pub mod read;
/// Traversal of an adjacency representation.
pub mod walk;
/// Writing SMILES string representations.
pub mod write;

pub use mendeleev::Element;
pub use mendeleev::Isotope;

mod doctests {
    #[cfg(doctest)]
    #[doc =include_str!("../README.md")]
    struct _ReadMe;
}
