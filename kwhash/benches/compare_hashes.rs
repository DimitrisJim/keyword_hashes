use criterion::{black_box, criterion_group, criterion_main, Criterion};
use kwhash::{get_token_match, get_token_match_len, get_token_phf, get_token_stdlib_hash};

// Simply split by whitespace.
//
// stdlib_testlib.txt is a dump of Python's stdlib and testlib names; it represents
// a good distribution of keywords and non-keywords.
fn rudimentary_tokens() -> Vec<&'static str> {
    let names = include_str!("stdlib_testlib.txt");
    names.split_whitespace().collect()
}

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
    let tokens: Vec<_> = rudimentary_tokens();

    bench_alt!(c, get_token_phf, "Rust-phf", &tokens);
    bench_alt!(c, get_token_stdlib_hash, "Standard library hashmap", &tokens);
    bench_alt!(c, get_token_match, "Match on keywords", &tokens);
    bench_alt!(c, get_token_match_len, "Match on keywords (pre match on length)", &tokens);
}

criterion_group!(benches, bench_alts);
criterion_main!(benches);
