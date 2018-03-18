pub fn factors(n: u64) -> Vec<u64> {
    let mut number = n;
    let mut factors = vec![];
    let mut candidate = 2;

    while number > 1 {
        while number % candidate == 0 {
            factors.push(candidate);
            number /= candidate
        }
        candidate += 1
    }

    factors
}
