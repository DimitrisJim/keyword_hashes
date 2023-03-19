#![allow(clippy::new_without_default)]

// todo: also return the seed from here.
#[inline(always)]
pub(crate) fn build_table(keys: Vec<String>, values: Vec<String>) -> (Vec<Option<(String, String)>>, u64) {
    // 35 keywords, use a power of two for the table size.
    let (attempts, mut table_size) = (50000000, 64);
    // use a random seed to try and get a more uniform distribution.
    let mut seed = rand::random::<u64>();
    let mut collision = false;
    loop {
        // Use the hash and try and fill up the table, if at any point we collide,
        // increase the table size and try again.
        for _ in 0..attempts {
            let mut table = vec![None; table_size];
            for (key, value) in keys.iter().zip(values.iter()) {
                let hash = hash(key, seed);
                let index = hash as usize % (table_size - 1);
                if table[index].is_some() {
                    // Collision, increase the table size and try again.
                    seed = rand::random::<u64>();
                    collision = true;
                    break;
                }
                table[index] = Some((key.to_owned(), value.to_owned()));
            }
            if collision {
                collision = false;
                table_size *= 2;
                continue;
            }
            // We successfully filled the table, return it.
            return (table, seed);
        }
    }
}

#[inline(always)]
// Grab the first two bytes and the last byte of the string and just xor it with the seed.
// We could try and do something more involved that might result in a more uniform distribution.
// It's a tradeoff between build-speed + resulting size and hash speed. (I'm going for hash speed now).
// todo: assumes minimum length of 2
pub(crate) fn hash(s: &str, seed: u64) -> u64 {
    let bytes = s.as_bytes();
    let hash =
        (bytes[0] as u64) << 24 | (bytes[1] as u64) << 16 | (bytes[bytes.len() - 1] as u64) << 8;
    hash ^ seed
}

#[allow(unused)]
pub mod map {
    /// This is what we output from DisplayMap and what we eventually load into our
    /// module.
    pub struct Map<K: 'static, V: 'static> {
        values: &'static [Option<(K, V)>],
        seed: u64,
    }

    impl<K, V> Map<K, V> {
        #[inline]
        pub const fn new() -> Self {
            Self { values: &[], seed: 0 }
        }

        pub fn get<T: ?Sized>(&self, _key: &T) -> Option<&V> {
            // Grab the hash and index values.
            unimplemented!()
        }
    }
}

#[allow(unused)]
pub mod codegen {
    use super::build_table;

    use std::collections::HashSet;
    use std::fmt;
    use std::hash::Hash;

    // This is what we use in our build script; builds a DisplayMap which on
    // fmt will output the code for creating map::Map.
    pub struct Map {
        keys: Vec<String>,
        values: Vec<String>,
    }

    impl Map {
        pub fn new() -> Map {
            Map {
                keys: vec![],
                values: vec![],
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
            let (table, seed) = build_table(self.keys.clone(), self.values.clone());
            println!("{:?}", table.len());
            DisplayMap { values: table, seed }
        }
    }

    /// An adapter for printing a [`Map`](Map).
    pub struct DisplayMap {
        values: Vec<Option<(String, String)>>,
        seed: u64,
    }

    impl fmt::Display for DisplayMap {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            // TODO: Write out the map properly
            write!(f, "tinyphf::Map {{{0}, {1}}}", self.values.len(), self.seed)
        }
    }
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
