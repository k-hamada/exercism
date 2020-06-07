use itertools::Itertools;
use std::collections::HashMap;

pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    permutation(extract(input))
        .find(|context| check(input, &context))
        .map(|context| context.iter().cloned().collect::<_>())
}

fn extract(input: &str) -> Vec<char> {
    input
        .chars()
        .filter(|char| char.is_alphabetic())
        .unique()
        .collect::<Vec<_>>()
}

fn permutation(input: Vec<char>) -> impl Iterator<Item = Vec<(char, u8)>> {
    (0..10)
        .permutations(input.len())
        .map(move |perm| input.iter().cloned().zip(perm).collect::<_>())
}

// context: HashMap<char, u8> is Heavy
fn check(input: &str, context: &[(char, u8)]) -> bool {
    let mut res = vec![];
    let mut current = 0u32;
    let mut tmp = 0u32;
    for c in input.chars() {
        match (tmp, c) {
            (0, '+') => return false,
            (0, '=') => continue,
            (_, ' ') => continue,
            (_, '+') => {
                current += tmp;
                tmp = 0;
            }
            (_, '=') => {
                current += tmp;
                tmp = 0;
                res.push(current);
                current = 0;
            }
            (_, _) => {
                if let Some((_, n)) = context.iter().find(|x| x.0 == c) {
                    if tmp == 0 && *n == 0 {
                        return false;
                    }
                    tmp *= 10;
                    tmp += *n as u32;
                }
            }
        }
    }
    res.push(tmp);
    res.len() > 1 && res.iter().all_equal()
}
