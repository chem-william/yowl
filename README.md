[![Codecov](https://codecov.io/github/chem-william/yowl/coverage.svg?branch=main)](https://codecov.io/gh/chem-william/yowl)
[![dependency status](https://deps.rs/repo/github/chem-william/yowl/status.svg)](https://deps.rs/repo/github/chem-william/yowl)

# Yowl

**Primitives for reading and writing SMILES strings in Rust.**
This project is a hard fork of [Purr](https://github.com/rapodaca/purr) and extends its functionality to support additional SMILES inputs accepted by RDKit and beyond.

## About

Yowl provides a safe, ergonomic API to parse and serialize molecular structures in the [OpenSMILES](https://opensmiles.org/opensmiles.html) format. SMILES (Simplified Molecular Input Line Entry System) is a widely adopted notation for representing molecular graphs as text strings.

## Usage

Add `yowl` to your `Cargo.toml`:

```toml
[dependencies]
yowl = "0.1"
```

## Examples

Parse acetamide into an adjacency representation:

```rust
use yowl::graph::{Builder, Atom, Bond};
use yowl::feature::{AtomKind, BondKind, Aliphatic};
use yowl::read::{read, Error};

fn main() -> Result<(), Error> {
    let mut builder = Builder::default();

    read("CC(=O)N", &mut builder, None)?;

    assert_eq!(builder.build(), Ok(vec![
        Atom {
            kind: AtomKind::Aliphatic(Aliphatic::C),
            bonds: vec![
                Bond::new(BondKind::Elided, 1)
            ]
        },
        Atom {
            kind: AtomKind::Aliphatic(Aliphatic::C),
            bonds: vec![
                Bond::new(BondKind::Elided, 0),
                Bond::new(BondKind::Double, 2),
                Bond::new(BondKind::Elided, 3)
            ]
        },
        Atom {
            kind: AtomKind::Aliphatic(Aliphatic::O),
            bonds: vec![
                Bond::new(BondKind::Double, 1)
            ]
        },
        Atom {
            kind: AtomKind::Aliphatic(Aliphatic::N),
            bonds: vec![
                Bond::new(BondKind::Elided, 1)
            ]
        }
    ]));

    Ok(())
}
```

The order of atoms and their substituents reflects their implied order within the corresponding SMILES string. This is important when atomic configuration (e.g., `@`, `@@`) is present at an atom.

An optional `Trace` type maps adjacency features to a cursor position in the original string. This is useful for conveying semantic errors such as hypervalence. 

```rust
use yowl::graph::Builder;
use yowl::read::{read, Error, Trace};

fn main() -> Result<(), Error> {
    let mut builder = Builder::default();
    let mut trace = Trace::new();

    //    012345678901234
    read("C(C)C(C)(C)(C)C", &mut builder, Some(&mut trace))?;

    // Texas carbon @ atom(2) with cursor range 4..5
    assert_eq!(trace.atom(2), Some(4..5));

    Ok(())
}
```

Syntax errors are mapped to the cursor at which they occur.

```rust
use yowl::graph::Builder;
use yowl::read::{read, Error};

fn main() {
    let mut builder = Builder::default();

    assert_eq!(read("OCCXC", &mut builder, None), Err(Error::Character(3)));
}
```

An adjacency can be written using `write`.

```rust
use yowl::graph::{Builder, Atom, Bond};
use yowl::feature::{AtomKind, BondKind, Aliphatic};
use yowl::read::{read, Error};
use yowl::write::Writer;
use yowl::walk::walk;

fn main() -> Result<(), Error> {
    let mut builder = Builder::default();

    read("c1c([37Cl])cccc1", &mut builder, None)?;

    let atoms = builder.build().expect("atoms");
    let mut writer = Writer::default();

    walk(atoms, &mut writer).expect("walk");

    assert_eq!(writer.write(), "c(ccccc1[37Cl])1");

    Ok(())
}
```

The output string doesn't match the input string, although both represent the same molecule (Cl-37 chlorobenzene). `write` traces `atoms` in depth-first order, but the adjacency representation (`atoms`) lacks information about how the original SMILES tree was cut.

## Why a hard fork
The original author of Purr has [seemingly passed away](https://doi.org/10.59350/myaw4-dtg76) ([he shared a bit of his journey on his blog](https://depth-first.com/articles/2024/05/24/bridge-to-nowhere/)), and the library needed extensions to accept a broader set of SMILES inputs (e.g., RDKit-compatible strings). Yowl continues maintenance and adds new features.

## Contributing

Contributions are welcome! Please open an issue or pull request. Ensure you add tests for new functionality and follow Rust formatting conventions (`cargo fmt`).

## License

Yowl is distributed under the terms of the MIT License. See [LICENSE-MIT](LICENSE-MIT) and [COPYRIGHT](COPYRIGHT) for details.
