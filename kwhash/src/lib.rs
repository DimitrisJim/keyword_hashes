//! Export functions for benching.
mod token;
use crate::token::*;

static KEYWORDS: phf::Map<&'static str, Tok> = include!(concat!(env!("OUT_DIR"), "/keywords.rs"));

#[inline(always)]
pub fn get_token_phf(s: &str) -> Option<&Tok> {
    KEYWORDS.get(s)
}
use once_cell::sync::Lazy;
use std::collections::HashMap;

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
    let len = s.len() - 2;
    if len > 6 {
        return None;
    }

    match len {
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
