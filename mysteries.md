# A.K.A the compiler is messing with me

How can a reordering of the names in a use statement have such a drastic effect? I.e, this diff:

```diff
 use criterion::{black_box, criterion_group, criterion_main, Criterion};
 use kwhash::{
-    get_token_match, get_token_match_len, get_token_match_len_dist,
-    get_token_phf, get_token_stdlib_hash, get_token_tinyphf
+    get_token_match, get_token_match_len, get_token_match_len_dist, get_token_phf,
+    get_token_stdlib_hash, get_token_tinyphf,
 };
 ```

result in a 20% performance drop? The _only_ thing that changed here was that `get_token_phf` was moved to the end of the first line.

---

Write up that match thing, too.