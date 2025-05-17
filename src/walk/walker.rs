use std::collections::HashMap;

use super::{Error, Follower};
use crate::{
    feature::BondKind,
    graph::{Atom, Bond, JoinPool},
};

/// Performs a depth-first traversal of `graph`, emitting SMILES via the `Follower`.
pub fn walk<F: Follower>(graph: Vec<Atom>, follower: &mut F) -> Result<(), Error> {
    let size = graph.len();
    let mut atoms = graph.into_iter().enumerate().collect::<HashMap<_, _>>();
    let mut pool = JoinPool::new();

    for id in 0..size {
        if let Some(root_atom) = atoms.remove(&id) {
            walk_root(id, root_atom, size, &mut atoms, follower, &mut pool)?;
        }
    }
    Ok(())
}

fn walk_root<F: Follower>(
    root_id: usize,
    root_atom: Atom,
    size: usize,
    atoms: &mut HashMap<usize, Atom>,
    follower: &mut F,
    pool: &mut JoinPool,
) -> Result<(), Error> {
    let mut stack = Vec::new();
    let mut chain = vec![root_id];

    // Initialize stack with root bonds (rev so first bond is processed first)
    for bond in root_atom.bonds.into_iter().rev() {
        stack.push((root_id, bond));
    }
    follower.root(root_atom.kind);

    while let Some((sid, bond)) = stack.pop() {
        validate_bond_indices(sid, bond.tid, size)?;
        backtrack_and_pop(sid, &mut chain, follower);

        if let Some(mut child_atom) = atoms.remove(&bond.tid) {
            process_tree_edge(
                sid,
                &bond,
                &mut child_atom,
                follower,
                &mut stack,
                &mut chain,
            )?;
        } else {
            process_ring_edge(sid, &bond, pool, follower);
        }
    }
    Ok(())
}

/// Validate basic bond errors: unknown target or self-loop.
const fn validate_bond_indices(sid: usize, tid: usize, size: usize) -> Result<(), Error> {
    if tid >= size {
        Err(Error::UnknownTarget(sid, tid))
    } else if tid == sid {
        Err(Error::Loop(sid))
    } else {
        Ok(())
    }
}

/// Pop the chain back to `sid`, emitting branch closures as needed.
fn backtrack_and_pop<F: Follower>(sid: usize, chain: &mut Vec<usize>, follower: &mut F) {
    let mut to_pop = 0;
    while *chain.last().unwrap() != sid {
        chain.pop();
        to_pop += 1;
    }
    if to_pop > 0 {
        follower.pop(to_pop);
    }
}

/// Handle a tree edge: remove the back-bond, check stereochemistry, push new bonds, and extend.
fn process_tree_edge<F: Follower>(
    sid: usize,
    bond: &Bond,
    child: &mut Atom,
    follower: &mut F,
    stack: &mut Vec<(usize, Bond)>,
    chain: &mut Vec<usize>,
) -> Result<(), Error> {
    let mut back_bond = None;
    for (idx, out) in child.bonds.drain(..).enumerate().rev() {
        if out.tid == sid {
            // Stereochemistry inversion on even index
            if idx % 2 == 0 {
                child.kind.invert_configuration();
            }
            back_bond = Some(out);
        } else {
            stack.push((bond.tid, out));
        }
    }
    let back = back_bond.ok_or(Error::HalfBond(sid, bond.tid))?;

    check_bond_compatibility(bond, &back)?;

    chain.push(bond.tid);

    // we elide single bonds, but keep the rest
    match bond.kind {
        BondKind::Single => follower.extend(BondKind::Elided, child.kind),
        _ => follower.extend(bond.kind, child.kind),
    }

    Ok(())
}

/// Ensure the forward and back bonds match, respecting directionality.
fn check_bond_compatibility(fwd: &Bond, back: &Bond) -> Result<(), Error> {
    if fwd.is_directional() {
        if fwd.kind == back.kind.reverse() {
            Ok(())
        } else {
            Err(Error::IncompatibleBond(fwd.tid, back.tid))
        }
    } else if fwd.kind != back.kind {
        Err(Error::IncompatibleBond(fwd.tid, back.tid))
    } else {
        Ok(())
    }
}

/// Handle a ring edge: allocate or retrieve a ring number and join.
fn process_ring_edge<F: Follower>(sid: usize, bond: &Bond, pool: &mut JoinPool, follower: &mut F) {
    let ring_id = pool.hit(sid, bond.tid);
    // we force elision of single bonds as we're within a ring
    match bond.kind {
        BondKind::Single => follower.join(BondKind::Elided, ring_id),
        _ => follower.join(bond.kind, ring_id),
    }
}

#[cfg(test)]
mod tests {
    use crate::Element;

    use super::*;
    use crate::feature::{AtomKind, BondKind, Symbol};
    use crate::graph::Bond;
    use crate::write::Writer;

    /// Simple linear C–O: should emit "CO"
    #[test]
    fn test_simple_linear() {
        let mut writer = Writer::default();
        let graph = vec![
            Atom {
                kind: AtomKind::Symbol(Symbol::Aliphatic(Element::C)),
                bonds: vec![Bond::new(BondKind::Elided, 1)],
            },
            Atom {
                kind: AtomKind::Symbol(Symbol::Aliphatic(Element::O)),
                bonds: vec![Bond::new(BondKind::Elided, 0)],
            },
        ];
        walk(graph, &mut writer).unwrap();
        assert_eq!(writer.write(), "CO");
    }

    /// Two disconnected single atoms: C and O -> "C.O"
    #[test]
    fn test_disconnected_components() {
        let mut writer = Writer::default();
        let graph = vec![
            Atom {
                kind: AtomKind::Symbol(Symbol::Aliphatic(Element::C)),
                bonds: vec![],
            },
            Atom {
                kind: AtomKind::Symbol(Symbol::Aliphatic(Element::O)),
                bonds: vec![],
            },
        ];
        walk(graph, &mut writer).unwrap();
        assert_eq!(writer.write(), "C.O");
    }

    /// Four‐membered single‐bond ring: should emit "C1CCC1"
    #[test]
    fn test_four_member_ring() {
        let mut writer = Writer::default();
        let graph = vec![
            Atom {
                kind: AtomKind::Symbol(Symbol::Aliphatic(Element::C)),
                bonds: vec![
                    Bond::new(BondKind::Single, 1),
                    Bond::new(BondKind::Single, 3),
                ],
            },
            Atom {
                kind: AtomKind::Symbol(Symbol::Aliphatic(Element::C)),
                bonds: vec![
                    Bond::new(BondKind::Single, 0),
                    Bond::new(BondKind::Single, 2),
                ],
            },
            Atom {
                kind: AtomKind::Symbol(Symbol::Aliphatic(Element::C)),
                bonds: vec![
                    Bond::new(BondKind::Single, 1),
                    Bond::new(BondKind::Single, 3),
                ],
            },
            Atom {
                kind: AtomKind::Symbol(Symbol::Aliphatic(Element::C)),
                bonds: vec![
                    Bond::new(BondKind::Single, 0),
                    Bond::new(BondKind::Single, 2),
                ],
            },
        ];
        walk(graph, &mut writer).unwrap();
        assert_eq!(writer.write(), "C(CCC1)1");
    }

    #[test]
    fn five_membered_ring_with_single_double_bond() {
        //      C
        //    /  \
        // 1 C    C
        //   \   /
        //    C=C
        //    0
        let mut writer = Writer::default();
        let graph = vec![
            Atom {
                kind: AtomKind::Symbol(Symbol::Aliphatic(Element::C)),
                bonds: vec![
                    Bond::new(BondKind::Single, 1),
                    Bond::new(BondKind::Double, 4),
                ],
            },
            Atom {
                kind: AtomKind::Symbol(Symbol::Aliphatic(Element::C)),
                bonds: vec![
                    Bond::new(BondKind::Single, 0),
                    Bond::new(BondKind::Single, 2),
                ],
            },
            Atom {
                kind: AtomKind::Symbol(Symbol::Aliphatic(Element::C)),
                bonds: vec![
                    Bond::new(BondKind::Single, 1),
                    Bond::new(BondKind::Single, 3),
                ],
            },
            Atom {
                kind: AtomKind::Symbol(Symbol::Aliphatic(Element::C)),
                bonds: vec![
                    Bond::new(BondKind::Single, 2),
                    Bond::new(BondKind::Single, 4),
                ],
            },
            Atom {
                kind: AtomKind::Symbol(Symbol::Aliphatic(Element::C)),
                bonds: vec![
                    Bond::new(BondKind::Double, 0),
                    Bond::new(BondKind::Single, 3),
                ],
            },
        ];
        walk(graph, &mut writer).unwrap();
        assert_eq!(writer.write(), "C(CCCC=1)=1");
    }

    #[test]
    fn five_membered_ring_with_two_double_bonds() {
        //      C
        //    /  \
        // 1 C    C
        //   \\  //
        //    C-C
        //    0
        let mut writer = Writer::default();
        let graph = vec![
            Atom {
                kind: AtomKind::Symbol(Symbol::Aliphatic(Element::C)),
                bonds: vec![
                    Bond::new(BondKind::Double, 1),
                    Bond::new(BondKind::Single, 4),
                ],
            },
            Atom {
                kind: AtomKind::Symbol(Symbol::Aliphatic(Element::C)),
                bonds: vec![
                    Bond::new(BondKind::Double, 0),
                    Bond::new(BondKind::Single, 2),
                ],
            },
            Atom {
                kind: AtomKind::Symbol(Symbol::Aliphatic(Element::C)),
                bonds: vec![
                    Bond::new(BondKind::Single, 1),
                    Bond::new(BondKind::Single, 3),
                ],
            },
            Atom {
                kind: AtomKind::Symbol(Symbol::Aliphatic(Element::C)),
                bonds: vec![
                    Bond::new(BondKind::Single, 2),
                    Bond::new(BondKind::Double, 4),
                ],
            },
            Atom {
                kind: AtomKind::Symbol(Symbol::Aliphatic(Element::C)),
                bonds: vec![
                    Bond::new(BondKind::Double, 3),
                    Bond::new(BondKind::Single, 0),
                ],
            },
        ];
        walk(graph, &mut writer).unwrap();
        assert_eq!(writer.write(), "C(=CCC=C1)1");
    }

    /// Directional bonds: up/down should emit "*/*"
    #[test]
    fn test_directional_bond() {
        let mut writer = Writer::default();
        let graph = vec![
            Atom {
                kind: AtomKind::Symbol(Symbol::Star),
                bonds: vec![Bond::new(BondKind::Up, 1)],
            },
            Atom {
                kind: AtomKind::Symbol(Symbol::Star),
                bonds: vec![Bond::new(BondKind::Down, 0)],
            },
        ];
        walk(graph, &mut writer).unwrap();
        assert_eq!(writer.write(), "*/*");
    }
}
