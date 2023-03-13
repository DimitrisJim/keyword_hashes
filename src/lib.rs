//! Export functions for benching.
mod token;
use token::Tok;

// generated in build.rs, in gen_phf()
/// A map of keywords to their tokens.
pub static KEYWORDS: phf::Map<&'static str, Tok> =
    include!(concat!(env!("OUT_DIR"), "/keywords.rs"));

#[inline(always)]
pub fn phf_is_keyword(s: &str) -> bool {
    KEYWORDS.contains_key(s)
}