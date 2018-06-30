use std::collections::HashMap;

pub fn primes_up_to(n: usize) -> Vec<usize> {
    let hash_map: HashMap<usize, _> =
        (2..n+1)
            .flat_map(|i|
                (2..)
                    .map(move |x| x * i)
                    .take_while(|x| x <= &n)
            )
            .map(|x| (x, ()))
            .collect();
            
    (2..n+1)
        .filter(|i| !hash_map.contains_key(i))
        .collect()
}
