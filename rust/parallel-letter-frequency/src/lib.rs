use std::collections::HashMap;

pub fn frequency(input: &[&str], _worker_count: usize) -> HashMap<char, usize> {
    let mut hm = HashMap::new();
    for words in input {
        for char in words.to_string().to_lowercase().chars() {
            if !char.is_alphabetic() { continue; }

            let entry = hm.entry(char).or_insert(0);
            *entry += 1;
        }
    }
    hm
}
