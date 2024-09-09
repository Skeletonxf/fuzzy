use fuzzy_string_distance::levenshtein_distance;

fn main() {
    assert_eq!(1, levenshtein_distance(&"rust", &"rusty")); // insert y
    assert_eq!(3, levenshtein_distance(&"bug", &"")); // delete all characters
    assert_eq!(2, levenshtein_distance(&"typography", &"typpgrapy")); // fix both typos
}
