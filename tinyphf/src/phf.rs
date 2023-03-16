#![allow(clippy::new_without_default)]

#[allow(unused)]
pub mod map {
    /// This is what we output from DisplayMap and what we eventually load into our
    /// module.
    pub struct Map<K: 'static, V: 'static> {
        pub values: &'static [(K, V)],
    }

    impl<K, V> Map<K, V> {
        #[inline]
        pub const fn new() -> Self {
            Self { values: &[] }
        }

        pub fn get<T: ?Sized>(&self, _key: &T) -> Option<&V> {
            // Grab the hash and index values.
            unimplemented!()
        }
    }
}

#[allow(unused)]
pub mod codegen {
    use std::collections::HashSet;
    use std::fmt;
    use std::hash::Hash;

    // This is what we use in our build script; builds a DisplayMap which on
    // fmt will output the code for creating map::Map.
    pub struct Map {
        keys: Vec<String>,
        values: Vec<String>,
        path: String,
    }

    impl Map {
        pub fn new() -> Map {
            Map {
                keys: vec![],
                values: vec![],
                path: String::from("::altphf"),
            }
        }

        /// Adds an entry to the builder.
        pub fn entry(&mut self, key: &str, value: &str) -> &mut Map {
            self.keys.push(key.to_owned());
            self.values.push(value.to_owned());
            self
        }

        /// Calculate the indexes and return a struct implementing
        /// `fmt::Display` which will print the constructed `map::Map`.
        ///
        /// Panics if there are any duplicate keys.
        pub fn build(&self) -> DisplayMap {
            let mut set = HashSet::new();
            for key in &self.keys {
                if !set.insert(key) {
                    panic!("duplicate key `{}`", key);
                }
            }

            // TODO: Run the algorithm for finding the correct indexes for the
            // keys and then place the values in here.
            DisplayMap { values: &[] }
        }
    }

    /// An adapter for printing a [`Map`](Map).
    pub struct DisplayMap {
        values: &'static [(String, String)],
    }

    impl fmt::Display for DisplayMap {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            // TODO: Write out the map.
            unimplemented!()
        }
    }
}

#[allow(unused)]
// Grab the first two bytes and the last byte of the string
pub(crate) fn hash(s: &str, _seed: u64) -> u64 {
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
            "False", "None", "True", "and", "as", "assert", "async", "await", "break", "class",
            "continue", "def", "del", "elif", "else", "except", "finally", "for", "from", "global",
            "if", "import", "in", "is", "lambda", "nonlocal", "not", "or", "pass", "raise",
            "return", "try", "while", "with", "yield",
        ];

        let mut set = HashSet::new();
        for keyword in keywords.iter() {
            let hash = hash(keyword, 0);
            set.insert(hash);
        }
        assert_eq!(set.len(), keywords.len());
    }
}
