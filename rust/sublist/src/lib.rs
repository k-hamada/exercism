#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal
}

pub fn sublist<T: PartialEq>(a: &[T], b: &[T]) -> Comparison {
    if a == b {
        return Comparison::Equal
    }
    if is_sublist(a, b) {
        return Comparison::Sublist;
    }
    if is_sublist(b, a) {
        return Comparison::Superlist;
    }
    Comparison::Unequal
}

fn is_sublist<T: PartialEq>(a: &[T], b: &[T]) -> bool {
    if a.is_empty() { return true; }
    if b.is_empty() { return false; }
    for n in 0..a.len() {
        if b.get(n .. n + a.len()).map_or(false, |c| a == c) {
            return true;
        }
    }
    false
}
