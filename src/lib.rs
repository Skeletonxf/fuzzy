/// Returns the minimum number of single character insertions, deletions or substitutions
/// required to convert the source string to the target string, known as the Levenshtein distance.
///
/// This is like a fuzzy [Eq](std::cmp::Eq), where a distance of 0 means the strings are equal
/// and the distance can be up to the length of the longer string if they are completely unrelated.
///
/// See also:
/// - [Levenshtein distance](https://en.wikipedia.org/wiki/Levenshtein_distance)
pub fn levenshtein_distance(source: &str, target: &str) -> usize {
    // FIXME: Return immediately if source or target are empty to avoid indexing out of bounds

    // We'll have a matrix A of source length + 1 rows and target length + 1 columns
    // This stores the edit distances for prefixes of source and target from the empty string
    // through to the entire inputs.
    // A[0, 0] is therefore "" to "" which is 0, and A[source length + 1, target length + 1] is
    // the edit distance from source to target.
    // We only need to store two rows at a time so we never construct this matrix.

    let mut edit_distances = vec![0; target.len() + 1];
    // First row of edit distances are converting an empty string source to prefixes of length 0
    // to the entire target, "" to "" is 0 edits, "" to one character is one insertion, and so on.
    for (i, x) in edit_distances.iter_mut().enumerate() {
        *x = i;
    }

    for i in 0..source.len() {
        // Step through each subsequent row of the matrix of edit distances, each time looking at
        // a prefix of source one character longer
        let mut new_edit_distances = vec![0; target.len() + 1];
        // We're on the i+1 prefix of characters in source, so converting this to an empty string
        // (the 0 character prefix of target) is purely deletions equal to the length of the
        // source.
        new_edit_distances[0] = i + 1;

        for j in 0..target.len() {
            // Step through columns for the prefixes of target on this prefix of source row

            // TODO: Comments for reason on each of these
            let deletion = edit_distances[j + 1] + 1;
            let insertion = new_edit_distances[j] + 1;
            // FIXME: Need to use character length not byte length and index by chars here
            // this won't handle non ASCII inputs properly right now
            let substitution = if source[i..(i+1)] == target[j..(j+1)] {
                edit_distances[j]
            } else {
                edit_distances[j] + 1
            };

            new_edit_distances[j + 1] = std::cmp::min(
                deletion, std::cmp::min(insertion, substitution)
            );
        }

        edit_distances = new_edit_distances;
    }
    edit_distances[target.len()]
}

/// Returns the minimum number of single character insertions, deletions or substitutions
/// required to convert the source string to the target string, known as the Levenshtein distance,
/// ignoring ASCII case differences.
///
/// This is like a fuzzy [eq_ignore_ascii_case](std::str::eq_ignore_ascii_case), where a distance
/// of 0 means the strings are equal and the distance can be up to the length of the longer
/// string if they are completely unrelated.
///
/// See also:
/// - [Levenshtein distance](https://en.wikipedia.org/wiki/Levenshtein_distance)
pub fn levenshtein_distance_ignore_ascii_case(source: &str, target: &str) -> usize {
    levenshtein_distance(&source.to_ascii_lowercase(), &target.to_ascii_lowercase())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let kitten = "kitten";
        let sitting = "sitting";
        let result = levenshtein_distance(&kitten, &sitting);
        assert_eq!(result, 3);
    }
}
