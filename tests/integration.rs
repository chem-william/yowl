extern crate yowl;

use std::fs;

use yowl::feature::{AtomKind, BondKind, Symbol};
use yowl::graph::{Atom, Bond, Builder};
use yowl::read::read;
use yowl::walk::walk;
use yowl::write::Writer;
use yowl::Element;

use pretty_assertions::assert_eq;

macro_rules! roundtrip_smiles {
    ($smiles:expr) => {{
        let mut writer = Writer::default();
        let _ = read($smiles, &mut writer, None);
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
                kind: AtomKind::Symbol(Symbol::Aliphatic(Element::C)),
                bonds: vec![Bond::new(BondKind::Elided, 1)]
            },
            Atom {
                kind: AtomKind::Symbol(Symbol::Aliphatic(Element::O)),
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
        "[as]",        // Bracket aromatic As
        "[b]",         // Bracket aromatic B
        "[c]",         // Bracket aromatic C
        "[o]",         // Bracket aromatic O
        "[s]",         // Bracket aromatic S
        "[p]",         // Bracket aromatic P
        "c1ccc[se]1",  // Bracket aromatic Se
        "c1ccc[te]1",  // Bracket aromatic Te
        "[si]1cccc[si]1", // Bracket aromatic Si
        "[Db][Sg][Bh][Hs][Mt][Ds][Rg][Cn][Nh][Fl][Mc][Lv][Ts][Og]", // new names for new elements
        "C[Fe@TH](O)(Cl)F", // Unspecified TH stereochemistry
        "C[Fe@TB](O)(Cl)(Br)F", // Unspecified TB stereochemistry
        "C[Fe@SP](O)(Cl)F", // Unspecified SP stereochemistry
        "C5CCC(C14CCCC1C2CCCC23CCCC34)C5", // Bunch of rings
    ];

    for smiles in all_smiles {
        roundtrip_smiles!(smiles);
    }
}

// wikidata SMILES taken from https://github.com/hobofan/smiles-parser
// using the following query
//
// SELECT ?item ?itemLabel ?smiles
// WHERE
// {
//   ?item wdt:P31 wd:Q11173.
//   ?item wdt:P233 ?value.
//   ?item wdt:P233 ?smiles
//   SERVICE wikibase:label { bd:serviceParam wikibase:language "[AUTO_LANGUAGE],en". }
// }
// LIMIT 5000

#[test]
fn bunch_of_smiles() {
    let contents =
        fs::read_to_string("tests/parsed_smiles.smi").expect("'parse_smiles.smi' available");
    for smiles in contents.split_whitespace() {
        roundtrip_smiles!(smiles);
    }
}

#[test]
fn special_case_temp_iupac_names() {
    let mut writer = Writer::default();

    // old temporary IUPAC names names for new elements
    let smiles = "[Uun][Uuu][Uub][Uut][Uuq][Uup][Uuh][Uus][Uuo]";

    // new names for new elements
    let expected_smiles = "[Ds][Rg][Cn][Nh][Fl][Mc][Lv][Ts][Og]";

    let _ = read(smiles, &mut writer, None);
    let written_smiles = writer.write();
    assert_eq!(written_smiles, expected_smiles);
}

#[test]
#[should_panic(expected = "Conflicting stereochemistry (multiple ")]
fn invalid_stereochemistry() {
    let smiles = "C/C(\\F)=C/F";
    let mut builder = Builder::default();
    read(smiles, &mut builder, None).unwrap();
    builder.build().expect("atoms");
}

#[test]
fn valid_stereochemistry() {
    let smiles = "N/C(/O)=C(\\S)/F";
    let mut builder = Builder::default();
    read(smiles, &mut builder, None).unwrap();
    builder.build().expect("atoms");
}

#[test]
fn clockwise_smiles() {
    let smiles = "F[C@H](Cl)(Br)";
    let mut builder = Builder::default();
    read(smiles, &mut builder, None).unwrap();
    dbg!(&builder);
    builder.build().expect("atoms");
}

#[test]
fn counterclockwise_smiles() {
    let smiles = "F[C@@H](Cl)(Br)";
    let mut builder = Builder::default();
    read(smiles, &mut builder, None).unwrap();
    let atoms = builder.build().expect("atoms");
    let mut writer = Writer::default();
    dbg!(&atoms);
    let _ = walk(atoms, &mut writer);
    dbg!(writer.write());
}

#[test]
fn smiles_with_single_quotes_are_ignored() {
    let mut writer = Writer::default();

    let smiles_with_quotes = "['Lv']['Ts']['Og']";
    let smiles_without_quotes = "[Lv][Ts][Og]";

    // Both variants should parse successfully and roundtrip as the one without quotes
    let _ = read(smiles_with_quotes, &mut writer, None);
    let written = writer.write();
    assert_eq!(written, smiles_without_quotes);

    let mut writer = Writer::default();
    let _ = read(smiles_without_quotes, &mut writer, None);
    let written2 = writer.write();
    assert_eq!(written2, smiles_without_quotes);
}

#[test]
fn error_reporting_with_quotes_matches_original_input() {
    use yowl::read::ReadError;
    let mut writer = Writer::default();
    // The error should be reported at the correct position in the original string,
    // even if there are single quotes before the error.
    let smiles = "C['Lv']['Ts']['Og']_";
    let err = read(smiles, &mut writer, None).unwrap_err();
    assert_eq!(err, ReadError::Character(19));
}
