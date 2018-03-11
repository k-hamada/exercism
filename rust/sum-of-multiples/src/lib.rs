use std::collections::HashSet;

pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    factors.iter()
        .flat_map(|&n| (n..limit).filter(|i| i % n == 0).collect::<Vec<_>>())
        .collect::<HashSet<_>>().iter()
        .sum()
}
