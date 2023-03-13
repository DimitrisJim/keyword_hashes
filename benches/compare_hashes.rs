use criterion::{criterion_group, criterion_main, Criterion};
use kwhash::phf_is_keyword;

// Tokenize the pydump.txt file containing the full stdlib and the test lib.
// TODO: Maybe filter by idents because that's what we basically do.
fn tokenize_py() -> impl Iterator<Item = &'static str> {
    let pydump = include_str!("pydump.txt");
    pydump.split_whitespace()
}

// Bench alternatives:
fn bench_alts(c: &mut Criterion) {
    // phf
    let tokens: Vec<_> = tokenize_py().collect();
    c.bench_function("phf", |b| {
        let tokens = tokens.clone();
        b.iter(|| {
            for t in &tokens {
                phf_is_keyword(t);
            }
        })
    });
}

criterion_group!(benches, bench_alts);
criterion_main!(benches);
