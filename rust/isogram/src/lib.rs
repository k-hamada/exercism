use std::collections::HashSet;

const WITHOUT : [char; 2] = ['-', ' '];

pub fn check(sentence : &str) -> bool {
    let chars = sentence.to_lowercase().chars().filter(|c| !WITHOUT.contains(c)).collect::<Vec<_>>();
    chars.len() == chars.into_iter().collect::<HashSet<_>>().len()
}
