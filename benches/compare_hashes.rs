use criterion::{black_box, criterion_group, criterion_main, Criterion};
use kwhash::{
    get_token_match, get_token_match_len, get_token_match_len_dist, get_token_phf,
    get_token_stdlib_hash, get_token_tinyphf,
};

macro_rules! bench_alt {
    ($c:ident, $func:ident, $name:literal, $tokens:expr) => {
        $c.bench_function($name, |b| {
            b.iter(|| {
                for t in $tokens {
                    black_box($func(t));
                }
            })
        });
    };
}

// Bench alternatives:
fn bench_alts(c: &mut Criterion) {
    let tokens: Vec<_> = include_str!("../stdlib_testlib.txt")
        .split_whitespace()
        .collect();

    bench_alt!(c, get_token_phf, "Rust-phf", &tokens);
    bench_alt!(
        c,
        get_token_stdlib_hash,
        "Standard library hashmap",
        &tokens
    );
    bench_alt!(c, get_token_match, "Match on keywords", &tokens);
    bench_alt!(
        c,
        get_token_match_len,
        "Match on keywords (pre match on length)",
        &tokens
    );
    bench_alt!(
        c,
        get_token_match_len_dist,
        "Match on keywords (pre match on length, most frequent matches first)",
        &tokens
    );
    bench_alt!(c, get_token_tinyphf, "Tinyphf", &tokens);
}

criterion_group!(benches, bench_alts);
criterion_main!(benches);
