pub fn is_pangram(sentence: &str) -> bool {
    let mut chars = sentence.to_string()
        .chars()
        .filter_map(|c| if c.is_ascii_alphabetic() { Some(c.to_ascii_lowercase()) } else { None })
        .collect::<Vec<_>>();
    chars.sort();
    chars.dedup();
    chars.len() == 26
}
