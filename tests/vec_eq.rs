/// Are provided vec equal.
pub fn vec_eq<T: PartialEq>(a: Vec<T>, b: Vec<T>) -> bool {
    if a.len() != b.len() {
        return false;
    }
    for (i, statement) in a.iter().enumerate() {
        if statement != &b[i] {
            return false;
        }
    }
    true
}
