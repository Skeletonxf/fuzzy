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
}
