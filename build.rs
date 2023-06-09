#![allow(unused)]
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::{Path, PathBuf};

fn main() {
    let out_dir = PathBuf::from(std::env::var_os("OUT_DIR").unwrap());
    gen_phf(&out_dir);
    // TODO
    gen_alt_phf(&out_dir);
}

fn gen_phf(out_dir: &Path) {
    let mut kwds = phf_codegen::Map::new();
    let kwds = kwds
        // Alphabetical keywords:
        .entry("...", "Tok::Ellipsis")
        .entry("False", "Tok::False")
        .entry("None", "Tok::None")
        .entry("True", "Tok::True")
        // moreso "standard" keywords
        .entry("and", "Tok::And")
        .entry("as", "Tok::As")
        .entry("assert", "Tok::Assert")
        .entry("async", "Tok::Async")
        .entry("await", "Tok::Await")
        .entry("break", "Tok::Break")
        .entry("case", "Tok::Case")
        .entry("class", "Tok::Class")
        .entry("continue", "Tok::Continue")
        .entry("def", "Tok::Def")
        .entry("del", "Tok::Del")
        .entry("elif", "Tok::Elif")
        .entry("else", "Tok::Else")
        .entry("except", "Tok::Except")
        .entry("finally", "Tok::Finally")
        .entry("for", "Tok::For")
        .entry("from", "Tok::From")
        .entry("global", "Tok::Global")
        .entry("if", "Tok::If")
        .entry("import", "Tok::Import")
        .entry("in", "Tok::In")
        .entry("is", "Tok::Is")
        .entry("lambda", "Tok::Lambda")
        .entry("match", "Tok::Match")
        .entry("nonlocal", "Tok::Nonlocal")
        .entry("not", "Tok::Not")
        .entry("or", "Tok::Or")
        .entry("pass", "Tok::Pass")
        .entry("raise", "Tok::Raise")
        .entry("return", "Tok::Return")
        .entry("try", "Tok::Try")
        .entry("while", "Tok::While")
        .entry("with", "Tok::With")
        .entry("yield", "Tok::Yield")
        .build();
    writeln!(
        BufWriter::new(File::create(out_dir.join("keywords_phf.rs")).unwrap()),
        "{kwds}",
    )
    .unwrap();
}

fn gen_alt_phf(out_dir: &Path) {
    let mut kwds = tinyphf::codegen::Map::new();
    let kwds = kwds
        // Alphabetical keywords:
        .entry("...", "Tok::Ellipsis")
        .entry("False", "Tok::False")
        .entry("None", "Tok::None")
        .entry("True", "Tok::True")
        // moreso "standard" keywords
        .entry("and", "Tok::And")
        .entry("as", "Tok::As")
        .entry("assert", "Tok::Assert")
        .entry("async", "Tok::Async")
        .entry("await", "Tok::Await")
        .entry("break", "Tok::Break")
        .entry("case", "Tok::Case")
        .entry("class", "Tok::Class")
        .entry("continue", "Tok::Continue")
        .entry("def", "Tok::Def")
        .entry("del", "Tok::Del")
        .entry("elif", "Tok::Elif")
        .entry("else", "Tok::Else")
        .entry("except", "Tok::Except")
        .entry("finally", "Tok::Finally")
        .entry("for", "Tok::For")
        .entry("from", "Tok::From")
        .entry("global", "Tok::Global")
        .entry("if", "Tok::If")
        .entry("import", "Tok::Import")
        .entry("in", "Tok::In")
        .entry("is", "Tok::Is")
        .entry("lambda", "Tok::Lambda")
        .entry("match", "Tok::Match")
        .entry("nonlocal", "Tok::Nonlocal")
        .entry("not", "Tok::Not")
        .entry("or", "Tok::Or")
        .entry("pass", "Tok::Pass")
        .entry("raise", "Tok::Raise")
        .entry("return", "Tok::Return")
        .entry("try", "Tok::Try")
        .entry("while", "Tok::While")
        .entry("with", "Tok::With")
        .entry("yield", "Tok::Yield")
        .build();
    writeln!(
        BufWriter::new(File::create(out_dir.join("keywords_tiny_phf.rs")).unwrap()),
        "{kwds}",
    )
    .unwrap();
}
