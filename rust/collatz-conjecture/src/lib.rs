// return Some(x) where x is the number of steps required to reach 1
pub fn collatz(n: u64) -> Option<u64> {
    do_collatz(n, 0)
}

fn do_collatz(n: u64, count: u64) -> Option<u64> {
    match (n, n % 2) {
        (0, _) => None,
        (1, _) => Some(count),
        (i, 0) => do_collatz(i / 2, count + 1),
        (i, _) => do_collatz(i * 3 + 1, count + 1)
    }
}
