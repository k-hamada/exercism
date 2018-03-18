// return Some(x) where x is the number of steps required to reach 1
pub fn collatz(n: u64) -> Option<u64> {
    match n {
        0 => None,
        _ => Some(do_collatz(n, 0))
    }
}

fn do_collatz(n: u64, count: u64) -> u64 {
    if n == 1 { return count; }

    let m = if n % 2 == 0 { n / 2 } else { n * 3 + 1 };
    do_collatz(m, count + 1)
}
