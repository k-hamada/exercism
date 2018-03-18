pub fn find() -> Option<u32> {
    const N : u32 = 1000;
    for a in 1..N {
        for b in a..N-a {
            let c = N - a - b;
            if (a*a + b*b == c*c) {
                return Some(a * b * c)
            }
        }
    }
    return None
}
