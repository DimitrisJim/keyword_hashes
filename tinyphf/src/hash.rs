//! Hash specific for Python keywords. Can be built similarly to JS keywords by using
//! the first two characters and the last character of the keyword:
//! 
//! ```python
//! >>> len(set([i[0] + i[1] + i[-1] for i in keyword.kwlist])) == len(keyword.kwlist)
//! True
//! ```
//! 
#[allow(unused)]
// Grab the first two bytes and the last byte of the string
pub(crate) fn hash(s: &str, _seed: u64) -> u64 {
    // TODO: Remove this assertion, make it a runtime check.
    assert!(s.len() >= 2);
    let bytes = s.as_bytes();
    (bytes[0] as u64) << 24 | (bytes[1] as u64) << 16 | (bytes[bytes.len() - 1] as u64) << 8
}

#[cfg(test)]
mod tests {
    // Test that all keywords have a unique hash and that the hash doesn't panic
    #[test]
    fn test_keywords() {
        use super::hash;
        use std::collections::HashSet;

        let keywords = [
            "False",
            "None",
            "True",
            "and",
            "as",
            "assert",
            "async",
            "await",
            "break",
            "class",
            "continue",
            "def",
            "del",
            "elif",
            "else",
            "except",
            "finally",
            "for",
            "from",
            "global",
            "if",
            "import",
            "in",
            "is",
            "lambda",
            "nonlocal",
            "not",
            "or",
            "pass",
            "raise",
            "return",
            "try",
            "while",
            "with",
            "yield",
        ];

        let mut set = HashSet::new();
        for keyword in keywords.iter() {
            let hash = hash(keyword, 0);
            set.insert(hash);
        }
        assert_eq!(set.len(), keywords.len());
    }
}
