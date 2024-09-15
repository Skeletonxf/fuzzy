//! Fuzzy string comparisons using [Levenshtein distance](https://en.wikipedia.org/wiki/Levenshtein_distance)
//!
//! Whereas simple string comparison is very sensitive to typos, Levenshtein Distance gives the minimum number of single-character edits (insertions, deletions or substitutions) required to change one word into the other. This gives us a sliding scale between 0 (strings are identical) and the length of the longer string (strings are unrelated), which can be used for fuzzy comparisons that can be resilient to typos, minor mistakes, and inconsistent spelling.
//!
//! ```
//! use fuzzy_string_distance::levenshtein_distance;
//! assert_eq!(1, levenshtein_distance(&"rust", &"rusty")); // insert y
//! assert_eq!(3, levenshtein_distance(&"bug", &"")); // delete all characters
//! assert_eq!(2, levenshtein_distance(&"typography", &"typpgrapy")); // fix both typos
//! ```
//!

/// Returns the minimum number of single character insertions, deletions or substitutions
/// required to convert the source string to the target string, known as the Levenshtein distance.
///
/// This is like a fuzzy [Eq], where a distance of 0 means the strings are equal
/// and the distance can be up to the length of the longer string if they are completely unrelated.
///
/// See also:
/// - [Levenshtein distance](https://en.wikipedia.org/wiki/Levenshtein_distance)
///
/// Note, this compares strings on a unicode scalar value basis, as per [str::chars]. While
/// this comparison is less likely to cut a 'character' in two than a byte by byte basis, it
/// still does not compare grapheme clusters.
pub fn levenshtein_distance(source: &str, target: &str) -> usize {
    // If either input is empty then the shortest transformation is all deletions or insertions
    // from/to an empty string, which will be equal to the number of characters in the other input
    // This check also guards against any index out of bounds issues in the main implementation
    let target_chars = target.chars().count();
    let source_chars = source.chars().count();
    if source.is_empty() {
        return target_chars;
    }
    if target.is_empty() {
        return source_chars;
    }

    // We'll have a matrix A of `source` length + 1 rows and `target` length + 1 columns
    // This stores the edit distances for prefixes of source and target from the empty string
    // through to the entire inputs.
    // A[0, 0] is therefore "" to "" which is 0, and A[source length + 1, target length + 1] is
    // the edit distance from source to target.
    // We only need to store two rows at a time so we never construct this matrix.

    let mut edit_distances = vec![0; target_chars + 1];
    // First row of edit distances are converting an empty string `source` to prefixes of length 0
    // to the entire `target`, "" to "" is 0 edits, "" to one character is one insertion, and
    // so on through to the entire target string.
    for (i, x) in edit_distances.iter_mut().enumerate() {
        *x = i;
    }

    for i in 0..source_chars {
        // Step through each subsequent row of the matrix of edit distances, each time looking at
        // a prefix of `source` one character longer
        let mut new_edit_distances = vec![0; target_chars + 1];
        // We're on the i+1 prefix of characters in `source`, so converting this to an empty string
        // (the 0 character prefix of target) is purely deletions equal to the length of the
        // source.
        new_edit_distances[0] = i + 1;

        for j in 0..target_chars {
            // Step through columns for the prefixes of `target` on this prefix of `source` row.
            // For a source of "kitten" and a target of "sitting", if we were up to i = 1 and
            // j = 2 then this would look like a source of "ki" we already have the distance for
            // converting to "si" and we now need to work out the distance to convert to "sit".
            // We're now calculating the edit distance for A[i + 1, j + 1]

            // At A[i, j + 1] we have the cost to reach the same `target` prefix with a source
            // that was one character shorter, so we can delete the extraneous character and the
            // distance could be 1 greater
            let deletion = edit_distances[j + 1] + 1;
            // At A[i + 1, j] we have the cost to reach a shorter `target` prefix with the same
            // source, so we can insert the extra character and the distance could be 1 greater
            let insertion = new_edit_distances[j] + 1;
            // We can unwrap here because we're taking an element from both iterators within
            // their respective bounds of 0 to source_chars -1 and target_chars - 1
            let source_char = source.chars().skip(i).next().unwrap();
            let target_char = target.chars().skip(j).next().unwrap();
            let substitution = if source_char == target_char {
                // If the `source` character at i and the `target` character at j match, we
                // don't need to transform anything
                edit_distances[j]
            } else {
                // Otherwise we can transform the character to match the target, and the distance
                // could be 1 greater
                edit_distances[j] + 1
            };

            // We always pick the cheapest option from the 3 we could do, which populates
            // A[i + 1, j + 1]
            new_edit_distances[j + 1] = std::cmp::min(
                deletion, std::cmp::min(insertion, substitution)
            );
        }

        edit_distances = new_edit_distances;
    }
    // The distance from `target` to `source` will be the final entry in the array as this
    // is the full strings of both with no characters ignored.
    edit_distances[target_chars]
}

/// Returns the minimum number of single character insertions, deletions or substitutions
/// required to convert the source string to the target string, known as the Levenshtein distance,
/// ignoring ASCII case differences.
///
/// This is like a fuzzy [eq_ignore_ascii_case](str::eq_ignore_ascii_case), where a distance
/// of 0 means the strings are equal and the distance can be up to the length of the longer
/// string if they are completely unrelated.
///
/// See also:
/// - [Levenshtein distance](https://en.wikipedia.org/wiki/Levenshtein_distance)
///
/// Note, this compares strings on a unicode scalar value basis, as per [str::chars]. While
/// this comparison is less likely to cut a 'character' in two than a byte by byte basis, it
/// still does not compare grapheme clusters.
pub fn levenshtein_distance_ignore_ascii_case(source: &str, target: &str) -> usize {
    levenshtein_distance(&source.to_ascii_lowercase(), &target.to_ascii_lowercase())
}

/// A modified Levenshtein distance that matches from the source string to an arbitrary substring
/// of the target string, returning the minimum number of single character insertions, deletions
/// or substitutions required to convert the source string to match any substring in the target.
///
/// This is like a fuzzy [str::contains], where a distance of 0 means the source string is equal
/// to a substring in the target and the distance can be up to the length of the longer string
/// if they are completely unrelated.
///
/// It can help to think of the source string being a query to search against a list of items each
/// with a longer target string, short searches that match exactly against part of the target
/// string will have the minimum number of edits 0, and as a few characters need to be modified to
/// match against part of the target the distances will increase.
///
/// ```
/// use fuzzy_string_distance::local_levenshtein_distance;
/// // trivial match to substring
/// assert_eq!(0, local_levenshtein_distance(&"long", &"A long sentence"));
/// // local distance is asymmetric, here we have to delete almost all the search term
/// assert_eq!(11, local_levenshtein_distance(&"A long sentence", &"long"));
/// ```
///
/// See also:
/// - [Levenshtein distance](https://en.wikipedia.org/wiki/Levenshtein_distance)
/// - [Fuzzy Substring Matching: On-device Fuzzy Friend Search at Snapchat](http://arxiv.org/pdf/2211.02767)
///
/// Note, this compares strings on a unicode scalar value basis, as per [str::chars]. While
/// this comparison is less likely to cut a 'character' in two than a byte by byte basis, it
/// still does not compare grapheme clusters.
pub fn local_levenshtein_distance(source: &str, target: &str) -> usize {
    // If either input is empty then the shortest transformation is all deletions or insertions
    // from/to an empty string.
    // This check also guards against any index out of bounds issues in the main implementation
    let target_chars = target.chars().count();
    let source_chars = source.chars().count();
    if source.is_empty() {
        // We can trivially match a 0 length substring in target with no edits
        return 0;
    }
    if target.is_empty() {
        // We only allow matches against substrings of the target, so we still need to delete all
        // the source characters if matching against "".
        return source_chars;
    }

    // We'll have a matrix A of `source` length + 1 rows and `target` length + 1 columns
    // This stores the edit distances for prefixes of source and target from the empty string
    // through to the entire inputs.
    // A[0, 0] is therefore "" to "" which is 0, and A[source length + 1, target length + 1] is
    // the edit distance from source to target.
    // We only need to store two rows at a time so we never construct this matrix.

    let mut edit_distances = vec![0; target_chars + 1];
    // Unlike in Levenshtein distance, we do not initialise the first row of the edit distances
    // to non zero values. These distances for converting an empty string `source` to prefixes of
    // length 0 to the entire `target`, "" to "" are all zero, as we do not want to penalise
    // starting a match further into the string. If the target was "racecar" and the source was
    // "car" this should also be 0 edits.

    for i in 0..source_chars {
        // Step through each subsequent row of the matrix of edit distances, each time looking at
        // a prefix of `source` one character longer
        let mut new_edit_distances = vec![0; target_chars + 1];
        // We're on the i+1 prefix of characters in `source`, so converting this to an empty string
        // (the 0 character prefix of target) is purely deletions equal to the length of the
        // source.
        new_edit_distances[0] = i + 1;

        for j in 0..target_chars {
            // Step through columns for the prefixes of `target` on this prefix of `source` row.
            // For a source of "kitten" and a target of "sitting", if we were up to i = 1 and
            // j = 2 then this would look like a source of "ki" we already have the distance for
            // converting to "si" and we now need to work out the distance to convert to "sit".
            // We're now calculating the edit distance for A[i + 1, j + 1]

            // At A[i, j + 1] we have the cost to reach the same `target` prefix with a source
            // that was one character shorter, so we can delete the extraneous character and the
            // distance could be 1 greater
            let deletion = edit_distances[j + 1] + 1;
            // At A[i + 1, j] we have the cost to reach a shorter `target` prefix with the same
            // source, so we can insert the extra character and the distance could be 1 greater
            let insertion = new_edit_distances[j] + 1;
            // We can unwrap here because we're taking an element from both iterators within
            // their respective bounds of 0 to source_chars -1 and target_chars - 1
            let source_char = source.chars().skip(i).next().unwrap();
            let target_char = target.chars().skip(j).next().unwrap();
            let substitution = if source_char == target_char {
                // If the `source` character at i and the `target` character at j match, we
                // don't need to transform anything
                edit_distances[j]
            } else {
                // Otherwise we can transform the character to match the target, and the distance
                // could be 1 greater
                edit_distances[j] + 1
            };

            // We always pick the cheapest option from the 3 we could do, which populates
            // A[i + 1, j + 1]
            new_edit_distances[j + 1] = std::cmp::min(
                deletion, std::cmp::min(insertion, substitution)
            );
        }

        edit_distances = new_edit_distances;
    }
    // The distance from `target` to `source` will be the final entry in the array as this
    // is the full strings of both with no characters ignored. We just want to match against any
    // substring of target, so by taking the minimum value in the final row we allow arbitrary
    // suffixes in the target to be ignored for the match. Since we initialised the first row
    // to 0 to allow arbitrary prefixes of the target to be ignored for the match, together this
    // returns the smallest edits required to match any substring of target.
    edit_distances.into_iter().min().unwrap()
}

/// A modified Levenshtein distance that matches from the source string to an arbitrary substring
/// of the target string, returning the minimum number of single character insertions, deletions
/// or substitutions required to convert the source string to match any substring in the target,
/// ignoring ASCII case differences
///
/// This is like a fuzzy [str::contains] that ignores case, where a distance of 0 means the source
/// string is equal to a substring in the target and the distance can be up to the length of the
/// longer string if they are completely unrelated.
///
/// It can help to think of the source string being a query to search against a list of items each
/// with a longer target string, short searches that match exactly against part of the target
/// string will have the minimum number of edits 0, and as a few characters need to be modified to
/// match against part of the target the distances will increase.
///
/// ```
/// use fuzzy_string_distance::local_levenshtein_distance_ignore_ascii_case;
/// // trivial match to substring
/// assert_eq!(0, local_levenshtein_distance_ignore_ascii_case(&"LONG", &"A long sentence"));
/// // local distance is asymmetric, here we have to delete almost all the search term
/// assert_eq!(11, local_levenshtein_distance_ignore_ascii_case(&"A long sentence", &"LONG"));
/// ```
///
/// See also:
/// - [Levenshtein distance](https://en.wikipedia.org/wiki/Levenshtein_distance)
/// - [Fuzzy Substring Matching: On-device Fuzzy Friend Search at Snapchat](http://arxiv.org/pdf/2211.02767)
///
/// Note, this compares strings on a unicode scalar value basis, as per [str::chars]. While
/// this comparison is less likely to cut a 'character' in two than a byte by byte basis, it
/// still does not compare grapheme clusters.
pub fn local_levenshtein_distance_ignore_ascii_case(source: &str, target: &str) -> usize {
    local_levenshtein_distance(&source.to_ascii_lowercase(), &target.to_ascii_lowercase())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn transforming_input() {
        let kitten = "kitten";
        let sitting = "sitting";
        let result = levenshtein_distance(&kitten, &sitting);
        assert_eq!(result, 3);
    }

    #[test]
    fn adding_a_character() {
        let result = levenshtein_distance(&"rust", &"rusty");
        assert_eq!(result, 1);
    }

    #[test]
    fn removing_characters() {
        let result = levenshtein_distance(&"ferrisground", &"run");
        // run is present in the source input, so shortest transformation is removing the other
        // characters
        assert_eq!(result, 9);
    }

    #[test]
    fn empty_source() {
        let result = levenshtein_distance(&"", &"rust");
        assert_eq!(result, 4);
    }

    #[test]
    fn empty_target() {
        let result = levenshtein_distance(&"bug", &"");
        assert_eq!(result, 3);
    }

    #[test]
    fn multiple_transformations() {
        let result = levenshtein_distance(&"Edit distance", &"Eddy");
        // Edd already present in input, so can delete all the other characters and insert y,
        // so 3 edits fewer than the source input
        assert_eq!(result, 10);
    }

    #[test]
    fn unrelated() {
        let result = levenshtein_distance(&"unrelated", &"SCREAMING");
        assert_eq!(result, 9);
    }

    #[test]
    fn slightly_related_ignoring_case() {
        let result = levenshtein_distance_ignore_ascii_case(&"unrelated", &"SCREAMING");
        assert_eq!(result, 7);
    }

    #[test]
    fn non_english() {
        let result = levenshtein_distance(&"El delfín español", &"Dolphin");
        assert_eq!(result, 15);
    }

    #[test]
    fn graphemes() {
        let result = levenshtein_distance(&"🧑‍🔬", &"🧑");
        // Split scientist into just person emoji
        assert_eq!(result, 2);
    }

    #[test]
    fn non_english_local() {
        let result = local_levenshtein_distance(&"Dolphin", &"El delfín español");
        // delfín -> Dolphin is 5 edits
        assert_eq!(result, 5);
        // local distance is asymmetric, search term is going to have to be modified to match
        // entire target as with non local distance
        let result = local_levenshtein_distance(&"El delfín español", &"Dolphin");
        assert_eq!(result, 15);
    }

    #[test]
    fn search_term() {
        let result = local_levenshtein_distance(&"Piñata", &"Pinecone tree");
        // Pineco -> Piñata is 4 edits
        assert_eq!(result, 4);
    }

    #[test]
    fn no_search() {
        let result = local_levenshtein_distance(&"", &"A long sentence");
        // trivial match
        assert_eq!(result, 0);
        let result = local_levenshtein_distance(&"A long sentence", &"");
        // local distance is asymmetric, have to delete entire search term
        assert_eq!(result, 15);
    }

    #[test]
    fn one_character_term() {
        let result = local_levenshtein_distance(&"g", &"A long sentence");
        assert_eq!(result, 0);
    }

    #[test]
    fn slightly_related_ignoring_case_local() {
        let result = local_levenshtein_distance_ignore_ascii_case(&"SCREAMING", &"unrelated");
        assert_eq!(result, 7);
        let result = local_levenshtein_distance_ignore_ascii_case(&"SCREAM", &"unrelated");
        assert_eq!(result, 4);
    }
}
