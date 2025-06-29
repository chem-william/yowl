use super::{missing_character, read_bond, read_bracket, read_organic, read_rnum, Trace};
use crate::feature::{AtomKind, BondKind};
use crate::read::error::ReadError;
use crate::read::scanner::Scanner;
use crate::walk::Follower;

/// Reads a string using a `Follower` and optional `Trace`.
///
/// ```
/// use yowl::write::Writer;
/// use yowl::read::{read, Trace, ReadError};
///
/// fn main() -> Result<(), ReadError> {
///     let mut writer = Writer::default();
///     let mut trace = Trace::default();
///
///     read("CC(=O)N", &mut writer, Some(&mut trace))?;
///
///     assert_eq!(writer.write(), "CC(=O)N");
///     assert_eq!(trace.bond(1, 2), Some(3));
///
///     Ok(())
/// }
/// ```
pub fn read<F: Follower>(
    smiles: &str,
    follower: &mut F,
    mut trace: Option<&mut Trace>,
) -> Result<(), ReadError> {
    let mut scanner = Scanner::new(smiles);
    // Did we actually read something?
    let got_something = read_smiles(None, &mut scanner, follower, &mut trace)?.is_some();
    let at_end = scanner.is_done();

    match (got_something, at_end) {
        // Successfully read and consumed the whole string
        (true, true) => Ok(()),
        // Read nothing but exactly at end of input
        (false, true) => Err(ReadError::EndOfLine),
        // first: Read nothing and still have chars
        // second: Read something but there's leftover garbage
        (false | true, false) => Err(ReadError::Character(scanner.cursor())),
    }
}

// <smiles> ::= <atom> <body>*
fn read_smiles<F: Follower>(
    input: Option<BondKind>,
    scanner: &mut Scanner,
    follower: &mut F,
    trace: &mut Option<&mut Trace>,
) -> Result<Option<usize>, ReadError> {
    let cursor = scanner.cursor();
    let Some(atom_kind) = read_atom(scanner)? else {
        return Ok(None);
    };

    if let Some(bond_kind) = input {
        if let Some(trace) = trace {
            if bond_kind == BondKind::Elided {
                trace.extend(cursor, cursor..scanner.cursor());
            } else {
                trace.extend(cursor - 1, cursor..scanner.cursor());
            }
        }

        follower.extend(bond_kind, atom_kind);
    } else {
        follower.root(atom_kind);

        if let Some(trace) = trace {
            trace.root(cursor..scanner.cursor());
        }
    }
    let mut result = 1;

    loop {
        match read_body(scanner, follower, trace)? {
            Some(length) => result += length,
            None => break Ok(Some(result)),
        }
    }
}

// <atom> ::= <organic> | <bracket> | <star>
fn read_atom(scanner: &mut Scanner) -> Result<Option<AtomKind>, ReadError> {
    if let Some(organic) = read_organic(scanner)? {
        return Ok(Some(organic));
    }

    if let Some(bracket) = read_bracket(scanner)? {
        return Ok(Some(bracket));
    }

    Ok(None)
}

// <body> ::= <branch> | <split> | <union>
fn read_body<F: Follower>(
    scanner: &mut Scanner,
    follower: &mut F,
    trace: &mut Option<&mut Trace>,
) -> Result<Option<usize>, ReadError> {
    if read_branch(scanner, follower, trace)? {
        return Ok(Some(0));
    }

    if let Some(length) = read_split(scanner, follower, trace)? {
        return Ok(Some(length));
    }

    read_union(scanner, follower, trace)
}

// <branch> ::= "(" ( <dot> | <bond> )? <smiles> ")"
fn read_branch<F: Follower>(
    scanner: &mut Scanner,
    follower: &mut F,
    trace: &mut Option<&mut Trace>,
) -> Result<bool, ReadError> {
    match scanner.peek() {
        Some('(') => {
            scanner.pop();
        }
        _ => return Ok(false),
    }

    let length = if scanner.peek() == Some('.') {
        scanner.pop();

        match read_smiles(None, scanner, follower, trace)? {
            Some(length) => length,
            None => return Err(missing_character(scanner)),
        }
    } else {
        let bond_kind = read_bond(scanner);

        match read_smiles(Some(bond_kind), scanner, follower, trace)? {
            Some(length) => length,
            None => return Err(missing_character(scanner)),
        }
    };

    match scanner.peek() {
        Some(')') => {
            scanner.pop();
            follower.pop(length);

            if let Some(trace) = trace {
                trace.pop(length);
            }

            Ok(true)
        }
        _ => Err(missing_character(scanner)),
    }
}

// <split> ::= <dot> <smiles>
fn read_split<F: Follower>(
    scanner: &mut Scanner,
    follower: &mut F,
    trace: &mut Option<&mut Trace>,
) -> Result<Option<usize>, ReadError> {
    match scanner.peek() {
        Some('.') => {
            scanner.pop();
        }
        _ => return Ok(None),
    }

    (read_smiles(None, scanner, follower, trace)?).map_or_else(
        || Err(missing_character(scanner)),
        |length| Ok(Some(length)),
    )
}

// <union> ::= <bond>? ( <smiles> | <rnum> )
fn read_union<F: Follower>(
    scanner: &mut Scanner,
    follower: &mut F,
    trace: &mut Option<&mut Trace>,
) -> Result<Option<usize>, ReadError> {
    let bond_cursor = scanner.cursor();
    let bond_kind = read_bond(scanner);

    if let Some(length) = read_smiles(Some(bond_kind), scanner, follower, trace)? {
        return Ok(Some(length));
    }

    let cursor = scanner.cursor();

    match read_rnum(scanner)? {
        Some(rnum) => {
            if let Some(trace) = trace {
                trace.join(bond_cursor, cursor..scanner.cursor(), rnum);
            }

            follower.join(bond_kind, rnum);

            Ok(Some(0))
        }
        None => {
            if bond_kind == BondKind::Elided {
                Ok(None)
            } else {
                Err(missing_character(scanner))
            }
        }
    }
}

#[cfg(test)]
mod read {
    use super::*;
    use crate::write::Writer;
    use pretty_assertions::assert_eq;

    #[test]
    fn blank() {
        let mut writer = Writer::default();

        assert_eq!(read("", &mut writer, None), Err(ReadError::EndOfLine))
    }

    #[test]
    fn leading_paren() {
        let mut writer = Writer::default();

        assert_eq!(read("(", &mut writer, None), Err(ReadError::Character(0)))
    }

    #[test]
    fn invalid_tail() {
        let mut writer = Writer::default();

        assert_eq!(read("*?", &mut writer, None), Err(ReadError::Character(1)))
    }

    #[test]
    fn trailing_bond() {
        let mut writer = Writer::default();

        assert_eq!(read("*-", &mut writer, None), Err(ReadError::EndOfLine))
    }

    #[test]
    fn trailing_dot() {
        let mut writer = Writer::default();

        assert_eq!(read("*.", &mut writer, None), Err(ReadError::EndOfLine))
    }

    #[test]
    fn cut_percent_single_digit() {
        let mut writer = Writer::default();

        assert_eq!(
            read("*%1*", &mut writer, None),
            Err(ReadError::Character(3))
        )
    }

    #[test]
    fn open_paren_eol() {
        let mut writer = Writer::default();

        assert_eq!(read("*(", &mut writer, None), Err(ReadError::EndOfLine))
    }

    #[test]
    fn missing_close_paren() {
        let mut writer = Writer::default();

        assert_eq!(read("*(*", &mut writer, None), Err(ReadError::EndOfLine))
    }

    #[test]
    fn bond_to_invalid() {
        let mut writer = Writer::default();

        assert_eq!(read("*-X", &mut writer, None), Err(ReadError::Character(2)))
    }

    #[test]
    fn split_to_invalid() {
        let mut writer = Writer::default();

        assert_eq!(read("*.X", &mut writer, None), Err(ReadError::Character(2)))
    }

    #[test]
    fn branch_invalid() {
        let mut writer = Writer::default();

        assert_eq!(
            read("*(X)", &mut writer, None),
            Err(ReadError::Character(2))
        )
    }

    #[test]
    fn branch_rnum() {
        let mut writer = Writer::default();

        assert_eq!(
            read("*(1)*", &mut writer, None),
            Err(ReadError::Character(2))
        )
    }

    #[test]
    fn branch_bond_rnum() {
        let mut writer = Writer::default();

        assert_eq!(
            read("*(-1)*", &mut writer, None),
            Err(ReadError::Character(3))
        )
    }

    #[test]
    fn dot_rnum() {
        let mut writer = Writer::default();

        assert_eq!(read("*.1", &mut writer, None), Err(ReadError::Character(2)))
    }

    #[test]
    fn branch_split_invalid() {
        let mut writer = Writer::default();

        assert_eq!(
            read("*(.X)", &mut writer, None),
            Err(ReadError::Character(3))
        )
    }

    #[test]
    fn p1() {
        let mut writer = Writer::default();

        read("*", &mut writer, None).unwrap();

        assert_eq!(writer.write(), "*")
    }

    #[test]
    fn aliphatic_organic() {
        let mut writer = Writer::default();

        read("C", &mut writer, None).unwrap();

        assert_eq!(writer.write(), "C")
    }

    #[test]
    fn aromatic_organic() {
        let mut writer = Writer::default();

        read("c", &mut writer, None).unwrap();

        assert_eq!(writer.write(), "c")
    }

    #[test]
    fn bracket() {
        let mut writer = Writer::default();

        read("[CH4]", &mut writer, None).unwrap();

        assert_eq!(writer.write(), "[CH4]")
    }

    #[test]
    fn elided_rnum() {
        let mut writer = Writer::default();

        read("*1", &mut writer, None).unwrap();

        assert_eq!(writer.write(), "*1")
    }

    #[test]
    fn single_rnum() {
        let mut writer = Writer::default();

        read("*-1", &mut writer, None).unwrap();

        assert_eq!(writer.write(), "*-1")
    }

    #[test]
    fn p1_p1() {
        let mut writer = Writer::default();

        read("*.*", &mut writer, None).unwrap();

        assert_eq!(writer.write(), "*.*")
    }

    #[test]
    fn p1_p2_branched_inner() {
        let mut writer = Writer::default();

        read("*(.*)*", &mut writer, None).unwrap();

        assert_eq!(writer.write(), "*(.*)*")
    }

    #[test]
    fn p2() {
        let mut writer = Writer::default();

        read("*-*", &mut writer, None).unwrap();

        assert_eq!(writer.write(), "*-*")
    }

    #[test]
    fn p3() {
        let mut writer = Writer::default();

        read("**-*", &mut writer, None).unwrap();

        assert_eq!(writer.write(), "**-*")
    }

    #[test]
    fn p3_branched() {
        let mut writer = Writer::default();

        read("*(-*)=*", &mut writer, None).unwrap();

        assert_eq!(writer.write(), "*(-*)=*")
    }

    #[test]
    fn p4_branched_inside() {
        let mut writer = Writer::default();

        read("*(-**)=*", &mut writer, None).unwrap();

        assert_eq!(writer.write(), "*(-**)=*")
    }

    #[test]
    fn p4_branched_outside() {
        let mut writer = Writer::default();

        read("*(-*)=**", &mut writer, None).unwrap();

        assert_eq!(writer.write(), "*(-*)=**")
    }

    #[test]
    fn nested() {
        let mut writer = Writer::default();

        read("*(*(*-*)*)*", &mut writer, None).unwrap();

        assert_eq!(writer.write(), "*(*(*-*)*)*")
    }

    #[test]
    fn s4_inside() {
        let mut writer = Writer::default();

        read("*(-*)(=*)(#*)*", &mut writer, None).unwrap();

        assert_eq!(writer.write(), "*(-*)(=*)(#*)*")
    }

    #[test]
    fn s4_outside() {
        let mut writer = Writer::default();

        read("**(-*)(=*)*", &mut writer, None).unwrap();

        assert_eq!(writer.write(), "**(-*)(=*)*")
    }

    #[test]
    fn foo() {
        let mut writer = Writer::default();

        read("C(F)Cl", &mut writer, None).unwrap();

        assert_eq!(writer.write(), "C(F)Cl")
    }
}

#[cfg(test)]
mod trace {
    use super::*;
    use crate::write::Writer;
    use pretty_assertions::assert_eq;

    #[test]
    fn p1() {
        let mut trace = Trace::default();
        let mut writer = Writer::default();

        read("*", &mut writer, Some(&mut trace)).unwrap();

        assert_eq!(trace.atom(0), Some(0..1))
    }

    #[test]
    fn p2() {
        let mut trace = Trace::default();
        let mut writer = Writer::default();

        read("**", &mut writer, Some(&mut trace)).unwrap();

        assert_eq!(trace.atom(0), Some(0..1));
        assert_eq!(trace.atom(1), Some(1..2));
        assert_eq!(trace.bond(0, 1), Some(1));
        assert_eq!(trace.bond(1, 0), Some(1))
    }

    #[test]
    fn p2_single() {
        let mut trace = Trace::default();
        let mut writer = Writer::default();

        read("*-*", &mut writer, Some(&mut trace)).unwrap();

        assert_eq!(trace.atom(0), Some(0..1));
        assert_eq!(trace.atom(1), Some(2..3));
        assert_eq!(trace.bond(0, 1), Some(1));
        assert_eq!(trace.bond(1, 0), Some(1))
    }

    #[test]
    fn p3_branched() {
        let mut trace = Trace::default();
        let mut writer = Writer::default();

        //    01234
        read("*(*)*", &mut writer, Some(&mut trace)).unwrap();

        assert_eq!(trace.atom(0), Some(0..1));
        assert_eq!(trace.atom(1), Some(2..3));
        assert_eq!(trace.atom(2), Some(4..5));
        assert_eq!(trace.bond(0, 1), Some(2));
        assert_eq!(trace.bond(1, 0), Some(2));
        assert_eq!(trace.bond(0, 2), Some(4));
        assert_eq!(trace.bond(2, 0), Some(4))
    }

    #[test]
    fn c3() {
        let mut trace = Trace::default();
        let mut writer = Writer::default();

        //    01234
        read("*1**1", &mut writer, Some(&mut trace)).unwrap();

        assert_eq!(trace.atom(0), Some(0..1));
        assert_eq!(trace.atom(1), Some(2..3));
        assert_eq!(trace.atom(2), Some(3..4));
        assert_eq!(trace.bond(0, 1), Some(2));
        assert_eq!(trace.bond(1, 0), Some(2));
        assert_eq!(trace.bond(1, 2), Some(3));
        assert_eq!(trace.bond(2, 1), Some(3));
        assert_eq!(trace.bond(2, 0), Some(4));
    }
}
