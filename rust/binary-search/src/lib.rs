pub fn find(array: &[usize], key: usize) -> Option<usize> {
    find_impl(array, key, 0, array.len())
}

fn find_impl(array: &[usize], key: usize, min: usize, max: usize) -> Option<usize> {
    if max < min {
        return None;
    }

    let mid = min + (max - min) / 2;
    match (array.get(mid), mid) {
        (None, _) => None,
        (Some(&n), 0) if n > key => None,
        (Some(&n), _) if n > key => find_impl(array, key, min, mid - 1),
        (Some(&n), _) if n < key => find_impl(array, key, mid + 1, max),
        (_, _) => Some(mid)
    }
}
