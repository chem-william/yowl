use std::collections::HashMap;

use super::{reconcile, Atom, Bond, Error};
use crate::feature::{AtomKind, BondKind, Rnum};
use crate::walk::Follower;

/// A `Follower` that builds an adjacency list SMILES representation.
///
/// ```
/// use yowl::walk::Follower;
/// use yowl::graph::{Atom, Bond, Builder};
/// use yowl::feature::{AtomKind, BondKind, Symbol};
///
/// let mut builder = Builder::default();
///
/// builder.root(AtomKind::Symbol(Symbol::Star));
/// builder.extend(BondKind::Double, AtomKind::Symbol(Symbol::Star));
///
/// assert_eq!(builder.build(), Ok(vec![
///     Atom {
///         kind: AtomKind::Symbol(Symbol::Star),
///         bonds: vec![
///             Bond::new(BondKind::Double, 1)
///         ]
///     },
///     Atom {
///         kind: AtomKind::Symbol(Symbol::Star),
///         bonds: vec![
///             Bond::new(BondKind::Double, 0)
///         ]
///     }
/// ]))
/// ```
#[derive(Debug, PartialEq, Default)]
pub struct Builder {
    stack: Vec<usize>,
    graph: Vec<Node>,
    opens: HashMap<Rnum, (usize, usize)>,
    errors: Vec<Error>,
    ring_idx: usize,
}

impl Builder {
    /// Builds the representation created by using the `Follower` trait
    /// methods.
    pub fn build(self) -> Result<Vec<Atom>, Error> {
        if let Some(error) = self.errors.into_iter().next() {
            return Err(error);
        }

        self.graph
            .into_iter()
            .enumerate()
            .map(|(idx, node)| {
                node.check_stereo(idx);

                let bonds = node
                    .edges
                    .into_iter()
                    .map(|edge| match edge.target {
                        Target::Id(tid) => Ok(Bond::new(edge.kind, tid)),
                        Target::Rnum(ring_idx, _, _) => Err(Error::Rnum(ring_idx)),
                    })
                    .collect::<Result<Vec<_>, _>>()?;

                Ok(Atom {
                    kind: node.kind,
                    bonds,
                })
            })
            .collect()
    }
}

impl Follower for Builder {
    fn root(&mut self, kind: AtomKind) {
        self.stack.push(self.graph.len());
        self.graph.push(Node::parent(kind));
    }

    fn extend(&mut self, bond_kind: BondKind, mut atom_kind: AtomKind) {
        let sid = *self.stack.last().expect("last on stack");
        let tid = self.graph.len();
        let reverse = Edge::new(bond_kind.reverse(), Target::Id(sid));
        let forward = Edge::new(bond_kind, Target::Id(tid));

        atom_kind.invert_configuration();

        self.stack.push(self.graph.len());
        self.graph.push(Node::child(reverse, atom_kind));
        self.graph[sid].edges.push(forward);
    }

    fn join(&mut self, bond_kind: BondKind, rnum: Rnum) {
        // Get the current “source” atom index.
        let sid = *self.stack.last().expect("stack must be nonempty");

        // Attempt to take out any existing “open” entry for this ring number.
        // If there *was* an entry, `remove(&rnum)` returns `Some(tid)`, so we go into the
        // `if let Some(tid)` branch and “close” the ring.
        //
        // If there was *no* open entry, then `remove(&rnum)` returns `None`, so we
        // go into the `else` block and “open” the ring by inserting `rnum -> sid`.
        if let Some((tid, edge_idx)) = self.opens.remove(&rnum) {
            // --- “closing” the ring: ---
            //
            // We know that `tid` is the atom index where we previously left a
            // placeholder edge `Target::Rnum(...)`. We also know that `edge_idx`
            // is the edge where we placed the placeholder
            let edge = &mut self.graph[tid].edges[edge_idx];

            // Try to reconcile bond‐kinds. If they match up, rewrite the placeholder
            // to point back to `sid`, then add the complementary edge on `sid`.
            match reconcile(edge.kind, bond_kind) {
                Some((left_kind, right_kind)) => {
                    // Overwrite the placeholder on `tid` so it points to `sid` now:
                    edge.target = Target::Id(sid);
                    edge.kind = left_kind;

                    // And emit the partner edge from `sid` → `tid`:
                    self.graph[sid].add_edge(right_kind, Target::Id(tid));
                }
                None => {
                    // If we can’t reconcile, record an error instead.
                    self.errors.push(Error::Join(sid, tid));
                }
            }
        } else {
            // --- “opening” the ring for the first time: ---
            //
            // No previous entry existed for `rnum`, so store `rnum -> (sid, edge_idx)`
            // and emit a placeholder edge on `sid` that carries (ring_idx, sid, rnum).
            let placeholder_idx = self.graph[sid].edges.len();
            self.opens.insert(rnum, (sid, placeholder_idx));
            self.graph[sid].add_edge(bond_kind, Target::Rnum(self.ring_idx, sid, rnum));
        }

        // Bump the global ring‐ID counter no matter which branch we took.
        self.ring_idx += 1;
    }

    fn pop(&mut self, depth: usize) {
        for _ in 0..depth {
            self.stack.pop();
        }
    }
}

#[derive(Debug, PartialEq)]
struct Node {
    kind: AtomKind,
    edges: Vec<Edge>,
}

impl Node {
    const fn parent(kind: AtomKind) -> Self {
        Self {
            kind,
            edges: Vec::new(),
        }
    }

    fn child(input: Edge, kind: AtomKind) -> Self {
        Self {
            kind,
            edges: vec![input],
        }
    }

    fn add_edge(&mut self, kind: BondKind, target: Target) {
        self.edges.push(Edge::new(kind, target));
    }

    /// Ensure there’s at most one stereo-directional bond.
    ///
    /// # Panics
    /// Panic if there are >=2 [`BondKind::Up`] bonds or >=2 [`BondKind::Down`] bonds.
    fn check_stereo(&self, atom_idx: usize) {
        let mut up_count = 0;
        let mut down_count = 0;

        for edge in &self.edges {
            match edge.kind {
                BondKind::Up => {
                    up_count += 1;
                    assert!(
                        up_count <= 1,
                        "Conflicting stereochemistry (multiple Up bonds) \
                             at atom index {atom_idx}: {self:?}",
                    );
                }
                BondKind::Down => {
                    down_count += 1;
                    assert!(
                        down_count <= 1,
                        "Conflicting stereochemistry (multiple Down bonds) \
                             at atom index {atom_idx}: {self:?}",
                    );
                }
                _ => {}
            }
        }
    }
}

#[derive(Debug, PartialEq)]
struct Edge {
    kind: BondKind,
    target: Target,
}

impl Edge {
    const fn new(kind: BondKind, target: Target) -> Self {
        Self { kind, target }
    }
}

#[derive(Debug, PartialEq)]
enum Target {
    Id(usize),
    // ring_idx, sid, rnum
    Rnum(usize, usize, Rnum),
}

#[cfg(test)]
mod errors {
    use super::*;
    use crate::feature::Symbol;
    use pretty_assertions::assert_eq;

    #[test]
    fn join_incompatible() {
        let mut builder = Builder::default();

        builder.root(AtomKind::Symbol(Symbol::Star));
        builder.join(BondKind::Up, Rnum::new(1));
        builder.extend(BondKind::Elided, AtomKind::Symbol(Symbol::Star));
        builder.extend(BondKind::Elided, AtomKind::Symbol(Symbol::Star));
        builder.join(BondKind::Up, Rnum::new(1));

        assert_eq!(builder.build(), Err(Error::Join(2, 0)))
    }

    #[test]
    fn join_unbalanced() {
        let mut builder = Builder::default();

        builder.root(AtomKind::Symbol(Symbol::Star));
        builder.join(BondKind::Elided, Rnum::new(1));
        builder.extend(BondKind::Elided, AtomKind::Symbol(Symbol::Star));
        builder.extend(BondKind::Elided, AtomKind::Symbol(Symbol::Star));
        builder.join(BondKind::Elided, Rnum::new(1));
        builder.join(BondKind::Elided, Rnum::new(2));

        assert_eq!(builder.build(), Err(Error::Rnum(2)))
    }
}

#[cfg(test)]
mod build {
    use super::*;
    use crate::feature::{Configuration, Symbol, VirtualHydrogen};
    use pretty_assertions::assert_eq;

    #[test]
    fn p1() {
        let mut builder = Builder::default();

        builder.root(AtomKind::Symbol(Symbol::Star));

        assert_eq!(
            builder.build(),
            Ok(vec![Atom {
                kind: AtomKind::Symbol(Symbol::Star),
                bonds: vec![]
            }])
        )
    }

    #[test]
    fn p2() {
        let mut builder = Builder::default();

        builder.root(AtomKind::Symbol(Symbol::Star));
        builder.extend(BondKind::Elided, AtomKind::Symbol(Symbol::Star));

        assert_eq!(
            builder.build(),
            Ok(vec![
                Atom {
                    kind: AtomKind::Symbol(Symbol::Star),
                    bonds: vec![Bond::new(BondKind::Elided, 1)]
                },
                Atom {
                    kind: AtomKind::Symbol(Symbol::Star),
                    bonds: vec![Bond::new(BondKind::Elided, 0)]
                }
            ])
        )
    }

    #[test]
    fn p3() {
        let mut builder = Builder::default();

        builder.root(AtomKind::Symbol(Symbol::Star));
        builder.extend(BondKind::Elided, AtomKind::Symbol(Symbol::Star));
        builder.extend(BondKind::Single, AtomKind::Symbol(Symbol::Star));

        assert_eq!(
            builder.build(),
            Ok(vec![
                Atom {
                    kind: AtomKind::Symbol(Symbol::Star),
                    bonds: vec![Bond::new(BondKind::Elided, 1)]
                },
                Atom {
                    kind: AtomKind::Symbol(Symbol::Star),
                    bonds: vec![
                        Bond::new(BondKind::Elided, 0),
                        Bond::new(BondKind::Single, 2)
                    ]
                },
                Atom {
                    kind: AtomKind::Symbol(Symbol::Star),
                    bonds: vec![Bond::new(BondKind::Single, 1)]
                }
            ])
        )
    }

    #[test]
    fn p3_branched() {
        let mut builder = Builder::default();

        builder.root(AtomKind::Symbol(Symbol::Star));
        builder.extend(BondKind::Elided, AtomKind::Symbol(Symbol::Star));
        builder.pop(1);
        builder.extend(BondKind::Single, AtomKind::Symbol(Symbol::Star));

        assert_eq!(
            builder.build(),
            Ok(vec![
                Atom {
                    kind: AtomKind::Symbol(Symbol::Star),
                    bonds: vec![
                        Bond::new(BondKind::Elided, 1),
                        Bond::new(BondKind::Single, 2)
                    ]
                },
                Atom {
                    kind: AtomKind::Symbol(Symbol::Star),
                    bonds: vec![Bond::new(BondKind::Elided, 0)]
                },
                Atom {
                    kind: AtomKind::Symbol(Symbol::Star),
                    bonds: vec![Bond::new(BondKind::Single, 0)]
                }
            ])
        )
    }

    #[test]
    fn c3_elided_elided() {
        let mut builder = Builder::default();

        builder.root(AtomKind::Symbol(Symbol::Star));
        builder.join(BondKind::Elided, Rnum::new(1));
        builder.extend(BondKind::Elided, AtomKind::Symbol(Symbol::Star));
        builder.extend(BondKind::Elided, AtomKind::Symbol(Symbol::Star));
        builder.join(BondKind::Elided, Rnum::new(1));

        assert_eq!(
            builder.build(),
            Ok(vec![
                Atom {
                    kind: AtomKind::Symbol(Symbol::Star),
                    bonds: vec![
                        Bond::new(BondKind::Elided, 2),
                        Bond::new(BondKind::Elided, 1)
                    ]
                },
                Atom {
                    kind: AtomKind::Symbol(Symbol::Star),
                    bonds: vec![
                        Bond::new(BondKind::Elided, 0),
                        Bond::new(BondKind::Elided, 2)
                    ]
                },
                Atom {
                    kind: AtomKind::Symbol(Symbol::Star),
                    bonds: vec![
                        Bond::new(BondKind::Elided, 1),
                        Bond::new(BondKind::Elided, 0)
                    ]
                }
            ])
        )
    }

    #[test]
    fn c3_single_elided() {
        let mut builder = Builder::default();

        builder.root(AtomKind::Symbol(Symbol::Star));
        builder.join(BondKind::Single, Rnum::new(1));
        builder.extend(BondKind::Elided, AtomKind::Symbol(Symbol::Star));
        builder.extend(BondKind::Elided, AtomKind::Symbol(Symbol::Star));
        builder.join(BondKind::Elided, Rnum::new(1));

        assert_eq!(
            builder.build(),
            Ok(vec![
                Atom {
                    kind: AtomKind::Symbol(Symbol::Star),
                    bonds: vec![
                        Bond::new(BondKind::Single, 2),
                        Bond::new(BondKind::Elided, 1)
                    ]
                },
                Atom {
                    kind: AtomKind::Symbol(Symbol::Star),
                    bonds: vec![
                        Bond::new(BondKind::Elided, 0),
                        Bond::new(BondKind::Elided, 2)
                    ]
                },
                Atom {
                    kind: AtomKind::Symbol(Symbol::Star),
                    bonds: vec![
                        Bond::new(BondKind::Elided, 1),
                        Bond::new(BondKind::Single, 0)
                    ]
                }
            ])
        )
    }

    #[test]
    fn tetrahedral_root() {
        let mut builder = Builder::default();

        builder.root(AtomKind::Bracket {
            isotope: None,
            symbol: Symbol::Star,
            configuration: Some(Configuration::TH1),
            hcount: None,
            charge: None,
            map: None,
        });
        builder.extend(BondKind::Elided, AtomKind::Symbol(Symbol::Star));
        builder.pop(1);
        builder.extend(BondKind::Elided, AtomKind::Symbol(Symbol::Star));
        builder.pop(1);
        builder.extend(BondKind::Elided, AtomKind::Symbol(Symbol::Star));
        builder.pop(1);
        builder.extend(BondKind::Elided, AtomKind::Symbol(Symbol::Star));

        assert_eq!(
            builder.build(),
            Ok(vec![
                Atom {
                    kind: AtomKind::Bracket {
                        isotope: None,
                        symbol: Symbol::Star,
                        configuration: Some(Configuration::TH1),
                        hcount: None,
                        charge: None,
                        map: None
                    },
                    bonds: vec![
                        Bond::new(BondKind::Elided, 1),
                        Bond::new(BondKind::Elided, 2),
                        Bond::new(BondKind::Elided, 3),
                        Bond::new(BondKind::Elided, 4)
                    ]
                },
                Atom {
                    kind: AtomKind::Symbol(Symbol::Star),
                    bonds: vec![Bond::new(BondKind::Elided, 0)]
                },
                Atom {
                    kind: AtomKind::Symbol(Symbol::Star),
                    bonds: vec![Bond::new(BondKind::Elided, 0)]
                },
                Atom {
                    kind: AtomKind::Symbol(Symbol::Star),
                    bonds: vec![Bond::new(BondKind::Elided, 0)]
                },
                Atom {
                    kind: AtomKind::Symbol(Symbol::Star),
                    bonds: vec![Bond::new(BondKind::Elided, 0)]
                }
            ])
        )
    }

    #[test]
    fn tetrahedral_child() {
        let mut builder = Builder::default();

        builder.root(AtomKind::Symbol(Symbol::Star));
        builder.extend(
            BondKind::Elided,
            AtomKind::Bracket {
                isotope: None,
                symbol: Symbol::Star,
                configuration: Some(Configuration::TH1),
                hcount: None,
                charge: None,
                map: None,
            },
        );
        builder.extend(BondKind::Elided, AtomKind::Symbol(Symbol::Star));
        builder.pop(1);
        builder.extend(BondKind::Elided, AtomKind::Symbol(Symbol::Star));
        builder.pop(1);
        builder.extend(BondKind::Elided, AtomKind::Symbol(Symbol::Star));

        assert_eq!(
            builder.build(),
            Ok(vec![
                Atom {
                    kind: AtomKind::Symbol(Symbol::Star),
                    bonds: vec![Bond::new(BondKind::Elided, 1)]
                },
                Atom {
                    kind: AtomKind::Bracket {
                        isotope: None,
                        symbol: Symbol::Star,
                        configuration: Some(Configuration::TH1),
                        hcount: None,
                        charge: None,
                        map: None
                    },
                    bonds: vec![
                        Bond::new(BondKind::Elided, 0),
                        Bond::new(BondKind::Elided, 2),
                        Bond::new(BondKind::Elided, 3),
                        Bond::new(BondKind::Elided, 4)
                    ]
                },
                Atom {
                    kind: AtomKind::Symbol(Symbol::Star),
                    bonds: vec![Bond::new(BondKind::Elided, 1)]
                },
                Atom {
                    kind: AtomKind::Symbol(Symbol::Star),
                    bonds: vec![Bond::new(BondKind::Elided, 1)]
                },
                Atom {
                    kind: AtomKind::Symbol(Symbol::Star),
                    bonds: vec![Bond::new(BondKind::Elided, 1)]
                }
            ])
        )
    }

    #[test]
    fn tetrahedral_child_hydrogen() {
        let mut builder = Builder::default();

        builder.root(AtomKind::Symbol(Symbol::Star));
        builder.extend(
            BondKind::Elided,
            AtomKind::Bracket {
                isotope: None,
                symbol: Symbol::Star,
                configuration: Some(Configuration::TH1),
                hcount: Some(VirtualHydrogen::H1),
                charge: None,
                map: None,
            },
        );
        builder.extend(BondKind::Elided, AtomKind::Symbol(Symbol::Star));
        builder.pop(1);
        builder.extend(BondKind::Elided, AtomKind::Symbol(Symbol::Star));
        builder.pop(1);
        builder.extend(BondKind::Elided, AtomKind::Symbol(Symbol::Star));

        assert_eq!(
            builder.build(),
            Ok(vec![
                Atom {
                    kind: AtomKind::Symbol(Symbol::Star),
                    bonds: vec![Bond::new(BondKind::Elided, 1)]
                },
                Atom {
                    kind: AtomKind::Bracket {
                        isotope: None,
                        symbol: Symbol::Star,
                        configuration: Some(Configuration::TH2),
                        hcount: Some(VirtualHydrogen::H1),
                        charge: None,
                        map: None
                    },
                    bonds: vec![
                        Bond::new(BondKind::Elided, 0),
                        Bond::new(BondKind::Elided, 2),
                        Bond::new(BondKind::Elided, 3),
                        Bond::new(BondKind::Elided, 4)
                    ]
                },
                Atom {
                    kind: AtomKind::Symbol(Symbol::Star),
                    bonds: vec![Bond::new(BondKind::Elided, 1)]
                },
                Atom {
                    kind: AtomKind::Symbol(Symbol::Star),
                    bonds: vec![Bond::new(BondKind::Elided, 1)]
                },
                Atom {
                    kind: AtomKind::Symbol(Symbol::Star),
                    bonds: vec![Bond::new(BondKind::Elided, 1)]
                }
            ])
        )
    }
}
