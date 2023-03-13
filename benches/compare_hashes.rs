use criterion::{criterion_group, criterion_main, Criterion};
use kwhash::{phf, stdlib_hashmap};
use unic_ucd_ident::is_xid_start;

// Tokenize the pydump.txt file containing the full stdlib and the test lib.
// We only do a rudimentary tokenization by splitting on whitespace and filtering
// out non-identifiers. The effect of this is basically as if we had more non-keywords
// than what we usually would.
fn rudimentary_tokens() -> Vec<&'static str> {
    let pydump = include_str!("pydump.txt");
    pydump
        .split_whitespace()
        .filter(|s| {
            let c = s.chars().next().unwrap();
            match c {
                'a'..='z' | 'A'..='Z' | '_' => true,
                _ => is_xid_start(c),
            }
        })
        .collect()
}

// Bench alternatives:
fn bench_alts(c: &mut Criterion) {
    let tokens: Vec<_> = rudimentary_tokens();

    c.bench_function(&format!("phf: ({0} keywords)", tokens.len()), |b| {
        let tokens = tokens.clone();
        b.iter(|| {
            for t in &tokens {
                phf::is_keyword(t);
            }
        })
    });

    c.bench_function(&format!("stdlib hashmap: ({0} keywords)", tokens.len()), |b| {
        let tokens = tokens.clone();
        b.iter(|| {
            for t in &tokens {
                stdlib_hashmap::is_keyword(t);
            }
        })
    });
}

criterion_group!(benches, bench_alts);
criterion_main!(benches);
