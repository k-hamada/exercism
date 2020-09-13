use std::collections::HashMap;
use std::thread;

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    if input.is_empty() {
        return HashMap::new();
    }

    let chunk_size = (input.len() + worker_count - 1) / worker_count;

    let mut handlers = Vec::with_capacity(worker_count);
    for chunk in input.chunks(chunk_size) {
        let input = chunk.iter().map(|c| c.to_string()).collect::<Vec<_>>();
        let handle = thread::spawn(move || impl_frequency(input));
        handlers.push(handle);
    }

    let mut hashmap = HashMap::with_capacity(26);
    for handle in handlers {
        for (key, value) in handle.join().unwrap() {
            (*hashmap.entry(key).or_insert(0)) += value;
        }
    }
    hashmap
}

fn impl_frequency(input: Vec<String>) -> HashMap<char, usize> {
    let mut hashmap = HashMap::with_capacity(26);
    for line in input {
        for chr in line.chars() {
            if !chr.is_alphabetic() {
                continue;
            }
            *hashmap.entry(chr.to_ascii_lowercase()).or_insert(0) += 1;
        }
    }
    hashmap
}
