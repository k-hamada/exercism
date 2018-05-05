use std::collections::HashMap;

/// Count occurrences of words.
pub fn word_count(words: &str) -> HashMap<String, u32> {
    let mut map: HashMap<String, u32> = HashMap::new();
    words.split(|c: char| !c.is_alphanumeric() )
        .filter(|word| word.len() != 0 )
        .for_each(|word| *map.entry(word.to_lowercase().to_string()).or_insert(0) += 1 );
    map
}
