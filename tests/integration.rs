extern crate yowl;

use yowl::feature::{Aliphatic, AtomKind, BondKind};
use yowl::graph::{Atom, Bond, Builder};
use yowl::read::read;
use yowl::write::Writer;

use pretty_assertions::assert_eq;

macro_rules! roundtrip_smiles {
    ($smiles:expr) => {{
        let mut writer = Writer::default();
        let _ = read($smiles, &mut writer, None);
        // assert!(result.is_ok());
        let written_smiles = writer.write();
        assert_eq!(written_smiles, $smiles);
    }};
}

#[test]
fn it_works() {
    let mut builder = Builder::default();
    read("CO", &mut builder, None).unwrap();

    assert_eq!(
        builder.build(),
        Ok(vec![
            Atom {
                kind: AtomKind::Aliphatic(Aliphatic::C),
                bonds: vec![Bond::new(BondKind::Elided, 1)]
            },
            Atom {
                kind: AtomKind::Aliphatic(Aliphatic::O),
                bonds: vec![Bond::new(BondKind::Elided, 0)]
            }
        ])
    );
}

#[test]
fn roundtripping_smiles_strings() {
    let all_smiles = [
        "CO",                                                                        // Simple molecule
        "C1=CC=CC=C1",                                                               // Benzene
        "C[C@H](O)[C@@H](O)C(=O)O",                                                  // Lactic acid
        "C1CC1C(=O)O", // Cyclopropanecarboxylic acid
        "[Db][Sg][Bh][Hs][Mt][Ds][Rg][Cn][Nh][Fl][Mc][Lv][Ts][Og]", // Novel elements
        "O=Cc1ccc(O)c(OC)c1COc1cc(C=O)ccc1O", // Vanilin
        "CC(=O)NCCC1=CNc2c1cc(OC)cc2CC(=O)NCCc1c[nH]c2ccc(OC)cc12", // Melatonin
        "CC1=C(C(=O)C[C@@H]1OC(=O)[C@@H]2[C@H](C2(C)C)/C=C(\\C)/C(=O)OC)C/C=C\\C=C", // Pyrethrin II
        "OC[C@@H](O1)[C@@H](O)[C@H](O)[C@@H]2[C@@H]1c3c(O)c(OC)c(O)cc3C(=O)O2", // Bergenin
        "CC(=O)OCCC(/C)=C\\C[C@H](C(C)=C)CCC=C", // a pheromone of the Californian scale insect
        "CC[C@H](O1)CC[C@@]12CCCO2", // (2S,2R)-Chalgogran
        "OCCc1c(C)[n+](cs1)Cc2cnc(C)nc2N", // Thiamine
        "[as]",        // aromatic As
        "c1ccc[se]1",  // aromatic Se
        "c1ccc[te]1",  // aromatic Te
        "[si]1cccc[si]1", // aromatic Si
        "[Uun][Uuu][Uub][Uut][Uuq][Uup][Uuh][Uus][Uuo]", // old placeholder names for new elements
        "[Db][Sg][Bh][Hs][Mt][Ds][Rg][Cn][Nh][Fl][Mc][Lv][Ts][Og]", // new names for new elements
        "C[Fe@TH](O)(Cl)F", // Unspecified TH stereochemistry
        "C[Fe@TB](O)(Cl)(Br)F", // Unspecified TB stereochemistry
        "C[Fe@SP](O)(Cl)F", // Unspecified SP stereochemistry
    ];

    for smiles in all_smiles {
        roundtrip_smiles!(smiles);
    }
}
