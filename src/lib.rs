//! Export functions for benching.
mod token;
use crate::token::*;

use once_cell::sync::Lazy;
use std::collections::HashMap;

static KEYWORDS_PHF: phf::Map<&'static str, Tok> = include!(concat!(env!("OUT_DIR"), "/keywords_phf.rs"));

#[inline(always)]
pub fn get_token_phf(s: &str) -> Option<&Tok> {
    KEYWORDS_PHF.get(s)
}


// TODO: Fix up, add to benches.
// static KEYWORDS_TINYPHF: tinyphf::Map<&'static str, Tok> = include!(concat!(env!("OUT_DIR"), "/keywords_tiny_phf.rs"));
// #[inline(always)]
// pub fn get_token_tinyphf(s: &str) -> Option<&Tok> {
//     KEYWORDS_TINYPHF.get(s)
// }

static KEYWORDS_2: Lazy<HashMap<&'static str, Tok>> = Lazy::new(|| {
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
pub fn get_token_stdlib_hash(s: &str) -> Option<&Tok> {
    KEYWORDS_2.get(s)
}

#[inline(never)]
pub fn get_token_match(s: &str) -> Option<&Tok> {
    match s {
        "..." => Some(&Tok::Ellipsis),
        "False" => Some(&Tok::False),
        "None" => Some(&Tok::None),
        "True" => Some(&Tok::True),
        "and" => Some(&Tok::And),
        "as" => Some(&Tok::As),
        "assert" => Some(&Tok::Assert),
        "async" => Some(&Tok::Async),
        "await" => Some(&Tok::Await),
        "break" => Some(&Tok::Break),
        "case" => Some(&Tok::Case),
        "class" => Some(&Tok::Class),
        "continue" => Some(&Tok::Continue),
        "def" => Some(&Tok::Def),
        "del" => Some(&Tok::Del),
        "elif" => Some(&Tok::Elif),
        "else" => Some(&Tok::Else),
        "except" => Some(&Tok::Except),
        "finally" => Some(&Tok::Finally),
        "for" => Some(&Tok::For),
        "from" => Some(&Tok::From),
        "global" => Some(&Tok::Global),
        "if" => Some(&Tok::If),
        "import" => Some(&Tok::Import),
        "in" => Some(&Tok::In),
        "is" => Some(&Tok::Is),
        "lambda" => Some(&Tok::Lambda),
        "match" => Some(&Tok::Match),
        "nonlocal" => Some(&Tok::Nonlocal),
        "not" => Some(&Tok::Not),
        "or" => Some(&Tok::Or),
        "pass" => Some(&Tok::Pass),
        "raise" => Some(&Tok::Raise),
        "return" => Some(&Tok::Return),
        "try" => Some(&Tok::Try),
        "while" => Some(&Tok::While),
        "with" => Some(&Tok::With),
        "yield" => Some(&Tok::Yield),
        _ => None,
    }
}

// Pre-match on the length of the string before doing the match. This causes `match_keyword` to
// regress for some reason.
#[inline(always)]
pub fn get_token_match_len(s: &str) -> Option<&Tok> {
    match s.len() - 2 {
        0 => match s {
            "as" => Some(&Tok::As),
            "in" => Some(&Tok::In),
            "is" => Some(&Tok::Is),
            "if" => Some(&Tok::If),
            "or" => Some(&Tok::Or),
            _ => None,
        },
        1 => match s {
            "and" => Some(&Tok::And),
            "not" => Some(&Tok::Not),
            "for" => Some(&Tok::For),
            "try" => Some(&Tok::Try),
            "del" => Some(&Tok::Del),
            "def" => Some(&Tok::Def),
            "..." => Some(&Tok::Ellipsis),
            _ => None,
        },
        2 => match s {
            "case" => Some(&Tok::Case),
            "else" => Some(&Tok::Else),
            "from" => Some(&Tok::From),
            "pass" => Some(&Tok::Pass),
            "with" => Some(&Tok::With),
            "True" => Some(&Tok::True),
            "None" => Some(&Tok::None),
            _ => None,
        },
        3 => match s {
            "await" => Some(&Tok::Await),
            "break" => Some(&Tok::Break),
            "class" => Some(&Tok::Class),
            "False" => Some(&Tok::False),
            "async" => Some(&Tok::Async),
            "match" => Some(&Tok::Match),
            "raise" => Some(&Tok::Raise),
            "while" => Some(&Tok::While),
            "yield" => Some(&Tok::Yield),
            _ => None,
        },
        4 => match s {
            "assert" => Some(&Tok::Assert),
            "global" => Some(&Tok::Global),
            "import" => Some(&Tok::Import),
            "lambda" => Some(&Tok::Lambda),
            "return" => Some(&Tok::Return),
            "except" => Some(&Tok::Except),
            _ => None,
        },
        5 => match s {
            "finally" => Some(&Tok::Finally),
            _ => None,
        },
        6 => match s {
            "continue" => Some(&Tok::Continue),
            "nonlocal" => Some(&Tok::Nonlocal),
            _ => None,
        },
        _ => None,
    }
}

// Pre-match on the length of the string before doing the match and using the most
// frequent (based on stdlib/test code) keywords first. The idea is that after the
// jump the compiler does to a sub-region of the match space based on length, we'll
// give it the most likely keywords to match first.
//
// Update: According to the benchmarks, there isn't a statistically significant
// improvement. 
// Lesson: the compiler is obviously smarter on this than I am.
#[inline(always)]
pub fn get_token_match_len_dist(s: &str) -> Option<&Tok> {
    match s.len() - 2 {
        0 => match s {
            "if" => Some(&Tok::If),
            "in" => Some(&Tok::In),
            "is" => Some(&Tok::Is),
            "as" => Some(&Tok::As),
            "or" => Some(&Tok::Or),
            _ => None,
        },
        1 => match s {
            "def" => Some(&Tok::Def),
            "for" => Some(&Tok::For),
            "not" => Some(&Tok::Not),
            "try" => Some(&Tok::Try),
            "and" => Some(&Tok::And),
            "del" => Some(&Tok::Del),
            "..." => Some(&Tok::Ellipsis),
            _ => None,
        },
        2 => match s {
            "None" => Some(&Tok::None),
            "with" => Some(&Tok::With),
            "else" => Some(&Tok::Else),
            "True" => Some(&Tok::True),
            "from" => Some(&Tok::From),
            "pass" => Some(&Tok::Pass),
            // Not represented in sample.
            "case" => Some(&Tok::Case),
            _ => None,
        },
        3 => match s {
            "class" => Some(&Tok::Class),
            "raise" => Some(&Tok::Raise),
            "False" => Some(&Tok::False),
            "yield" => Some(&Tok::Yield),
            "while" => Some(&Tok::While),
            "break" => Some(&Tok::Break),
            "async" => Some(&Tok::Async),
            "await" => Some(&Tok::Await),
            // Not represented in sample.
            "match" => Some(&Tok::Match),
            _ => None,
        },
        4 => match s {
            "return" => Some(&Tok::Return),
            "import" => Some(&Tok::Import),
            "except" => Some(&Tok::Except),
            "lambda" => Some(&Tok::Lambda),
            "assert" => Some(&Tok::Assert),
            "global" => Some(&Tok::Global),
            _ => None,
        },
        5 => match s {
            "finally" => Some(&Tok::Finally),
            _ => None,
        },
        6 => match s {
            "continue" => Some(&Tok::Continue),
            "nonlocal" => Some(&Tok::Nonlocal),
            _ => None,
        },
        _ => None,
    }
}