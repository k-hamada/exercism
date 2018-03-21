extern crate num_bigint;
extern crate num_traits;
extern crate primal_check;
extern crate rand;

use num_bigint::ToBigInt;
use num_traits::cast::ToPrimitive;
use primal_check::miller_rabin as is_prime;
use rand::{thread_rng, Rng};

pub fn private_key(p: u64) -> u64 {
    let mut rng = thread_rng();
    loop {
        let k = rng.gen_range(2, p);
        if is_prime(k) { return k; }
    }
}

pub fn public_key(p: u64, g: u64, a: u64) -> u64 {
    modpow(g, a, p)
}

pub fn secret(p: u64, b_pub: u64, a: u64) -> u64 {
    modpow(b_pub, a, p)
}

fn modpow(base: u64, exponent: u64, modulus: u64) -> u64 {
    let b = ToBigInt::to_bigint(&base).unwrap();
    let e = ToBigInt::to_bigint(&exponent).unwrap();
    let m = ToBigInt::to_bigint(&modulus).unwrap();
    b.modpow(&e, &m).to_u64().unwrap()
}
