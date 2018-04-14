use std::cmp::Ordering;

#[derive(Debug, PartialEq, Eq)]
pub enum Classification {
    Abundant,
    Perfect,
    Deficient
}

pub fn classify(num: u64) -> Option<Classification> {
    if num <= 0 { return None; }

    match num.cmp(&factor_sum(num)) {
        Ordering::Less => Classification::Abundant,
        Ordering::Greater => Classification::Deficient,
        Ordering::Equal => Classification::Perfect,
    }.into()
}

fn factor_sum(num: u64) -> u64 {
    (1..num/2+1)
        .filter(|&i| num % i == 0)
        .sum()
}
