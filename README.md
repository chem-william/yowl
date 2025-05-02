# Yowl

**NOTE: This repository has been superseded by the [Balsa Reference Implementation](https://github.com/metamolecular/balsa/).**

Primitives for reading and writing the SMILES language in Rust.

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
yowl = "0.1"
```

## Examples

Parse acetamide into an adjacency representation:

```rust
use yowl::graph::{ Builder, Atom, Bond };
use yowl::feature::{ AtomKind, BondKind, Aliphatic };
use yowl::read::{ read, Error };

fn main() -> Result<(), Error> {
    let mut builder = Builder::new();

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
use yowl::read::{ read, Error, Trace };

fn main() -> Result<(), Error> {
    let mut builder = Builder::new();
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
use yowl::read::{ read, Error };

fn main() {
    let mut builder = Builder::new();

    assert_eq!(read("OCCXC", &mut builder, None), Err(Error::Character(3)));
}
```

An adjacency can be written using `write`.

```rust
use yowl::graph::{ Builder, Atom, Bond };
use yowl::feature::{ AtomKind, BondKind, Aliphatic };
use yowl::read::{ read, Error };
use yowl::write::Writer;
use yowl::walk::walk;

fn main() -> Result<(), Error> {
    let mut builder = Builder::new();

    read("c1c([37Cl])cccc1", &mut builder, None)?;

    let atoms = builder.build().expect("atoms");
    let mut writer = Writer::new();

    walk(atoms, &mut writer).expect("walk");

    assert_eq!(writer.write(), "c(ccccc1[37Cl])1");

    Ok(())
}
```

The output string doesn't match the input string, although both represent the same molecule (Cl-37 chlorobenzene). `write` traces `atoms` in depth-first order, but the adjacency representation (`atoms`) lacks information about how the original SMILES tree was cut.

# Versions

Yowl is not yet stable. Patch versions never introduce breaking changes, but minor/major versions probably will.

# License

Yowl is distributed under the terms of the MIT License. See
[LICENSE-MIT](LICENSE-MIT) and [COPYRIGHT](COPYRIGHT) for details.
