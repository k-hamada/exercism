pub fn raindrops(n: usize) -> String {
    let rem3 = n % 3 == 0;
    let rem5 = n % 5 == 0;
    let rem7 = n % 7 == 0;

    if !(rem3 || rem5 || rem7) {
        return n.to_string()
    }

    let mut s = String::new();
    if rem3 { s.push_str("Pling") }
    if rem5 { s.push_str("Plang") }
    if rem7 { s.push_str("Plong") }
    s
}
