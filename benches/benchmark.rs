use criterion::{criterion_group, criterion_main, Criterion};
use yowl::graph::Builder;
use yowl::read::read;

fn benchmark_smiles_parsing(c: &mut Criterion) {
    let smiles_strings = vec![
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
    ];

    c.bench_function("parse_smiles", |b| {
        b.iter(|| {
            for smiles in &smiles_strings {
                let mut builder = Builder::new();
                read(smiles, &mut builder, None).unwrap();
            }
        });
    });
}

criterion_group!(benches, benchmark_smiles_parsing);
criterion_main!(benches);
