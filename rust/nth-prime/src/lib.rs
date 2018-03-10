#![crate_type = "lib"]
#![crate_name = "nth_prime"]

pub fn nth(n: usize) -> Result<usize, usize> {
    if n <= 0 { return Err(n); }

    (2..)
        .filter(|&i| is_prime(i) )
        .nth(n - 1)
        .ok_or(n)
}

fn is_prime(n: usize) -> bool {
    if n <= 2 { return true; }
    if n % 2 == 0 { return false; }

    (2..)
        .take_while(|i| i * i <= n )
        .all(|i| n % i != 0 )
}
