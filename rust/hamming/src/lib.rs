pub fn hamming_distance(strand1: &str, strand2: &str) -> Result<usize, ()> {
    if strand1.len() != strand2.len() { return Err(()); }
    Ok(strand1.chars().zip(strand2.chars()).filter(|&(a, b)| a != b).count())
}
