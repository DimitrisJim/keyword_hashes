use criterion::{criterion_group, criterion_main, Criterion, black_box};
use kwhash::{phf, stdlib_hashmap, match_keyword};

// Simply split by whitespace. 
//
// stdlib_testlib.txt is a dump of Python's stdlib and testlib names; it represents
// a good distribution of keywords and non-keywords.
fn rudimentary_tokens() -> Vec<&'static str> {
    let names = include_str!("stdlib_testlib.txt");
    names
        .split_whitespace()
        .collect()
}

macro_rules! bench_alt {
    ($c:ident, $module:ident, $name:literal, $tokens:expr) => {
        $c.bench_function($name, |b| {
            b.iter(|| {
                for t in $tokens {
                    black_box($module::get_token(t));
                }
            })
        });
    };
}

// Bench alternatives:
fn bench_alts(c: &mut Criterion) {
    let tokens: Vec<_> = rudimentary_tokens();

    bench_alt!(c, phf, "Rust-phf", &tokens);
    bench_alt!(c, stdlib_hashmap, "Standard library hashmap", &tokens);
    bench_alt!(c, match_keyword, "Match on keywords", &tokens);
}

criterion_group!(benches, bench_alts);
criterion_main!(benches);
