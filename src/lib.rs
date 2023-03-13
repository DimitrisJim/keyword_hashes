//! Export functions for benching.
mod token;

// Rust phf option. Generated in build.rs, in gen_phf()
pub mod phf {
    use crate::token::*;

    pub static KEYWORDS: phf::Map<&'static str, Tok> =
        include!(concat!(env!("OUT_DIR"), "/keywords.rs"));

    #[inline(always)]
    pub fn is_keyword(s: &str) -> bool {
        KEYWORDS.contains_key(s)
    }
}

// Normal hashmap.
pub mod stdlib_hashmap {
    use crate::token::*;

    use once_cell::sync::Lazy;
    use std::collections::HashMap;

    static KEYWORDS: Lazy<HashMap<&'static str, Tok>> = Lazy::new(|| {
        let mut m = HashMap::new();
        m.insert("...", Tok::Ellipsis);
        m.insert("False", Tok::False);
        m.insert("None", Tok::None);
        m.insert("True", Tok::True);
        m.insert("and", Tok::And);
        m.insert("as", Tok::As);
        m.insert("assert", Tok::Assert);
        m.insert("async", Tok::Async);
        m.insert("await", Tok::Await);
        m.insert("break", Tok::Break);
        m.insert("case", Tok::Case);
        m.insert("class", Tok::Class);
        m.insert("continue", Tok::Continue);
        m.insert("def", Tok::Def);
        m.insert("del", Tok::Del);
        m.insert("elif", Tok::Elif);
        m.insert("else", Tok::Else);
        m.insert("except", Tok::Except);
        m.insert("finally", Tok::Finally);
        m.insert("for", Tok::For);
        m.insert("from", Tok::From);
        m.insert("global", Tok::Global);
        m.insert("if", Tok::If);
        m.insert("import", Tok::Import);
        m.insert("in", Tok::In);
        m.insert("is", Tok::Is);
        m.insert("lambda", Tok::Lambda);
        m.insert("match", Tok::Match);
        m.insert("nonlocal", Tok::Nonlocal);
        m.insert("not", Tok::Not);
        m.insert("or", Tok::Or);
        m.insert("pass", Tok::Pass);
        m.insert("raise", Tok::Raise);
        m.insert("return", Tok::Return);
        m.insert("try", Tok::Try);
        m.insert("while", Tok::While);
        m.insert("with", Tok::With);
        m.insert("yield", Tok::Yield);
        m
    });

    #[inline(always)]
    pub fn is_keyword(s: &str) -> bool {
        KEYWORDS.contains_key(s)
    }
}