# Fuzzy

Fuzzy string comparisons using [Levenshtein distance](https://en.wikipedia.org/wiki/Levenshtein_distance)

Whereas simple string comparison is very sensitive to typos, Levenshtein Distance gives the minimum number of single-character edits (insertions, deletions or substitutions) required to change one word into the other. This gives us a sliding scale between 0 (strings are identical) and the length of the longer string (strings are unrelated), which can be used for fuzzy comparisons that can be resilient to typos, minor mistakes, and inconsistent spelling.

```rust
use fuzzy_string_distance::levenshtein_distance;
assert_eq!(1, levenshtein_distance(&"rust", &"rusty")); // insert y
assert_eq!(3, levenshtein_distance(&"bug", &"")); // delete all characters
assert_eq!(2, levenshtein_distance(&"typography", &"typpgrapy")); // fix both typos
```

## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
