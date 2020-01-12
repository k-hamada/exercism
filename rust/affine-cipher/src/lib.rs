use itertools::join;
use modinverse::*;

const M: i32 = 26;
const ALPHABET: [char; 26] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z',
];

/// While the problem description indicates a return status of 1 should be returned on errors,
/// it is much more common to return a `Result`, so we provide an error type for the result here.
#[derive(Debug, Eq, PartialEq)]
pub enum AffineCipherError {
    NotCoprime(i32),
}

/// Encodes the plaintext using the affine cipher with key (`a`, `b`). Note that, rather than
/// returning a return code, the more common convention in Rust is to return a `Result`.
pub fn encode(plaintext: &str, a: i32, b: i32) -> Result<String, AffineCipherError> {
    if egcd(a, M).0 != 1 {
        return Err(AffineCipherError::NotCoprime(a));
    }

    Ok(join(
        plaintext
            .to_lowercase()
            .chars()
            .filter_map(|c| match c {
                c if char::is_alphabetic(c) => encode_impl(c, a, b).into(),
                c if char::is_numeric(c) => c.into(),
                _ => None,
            })
            .collect::<Vec<_>>()
            .chunks(5)
            .map(|chunk| chunk.iter().collect::<String>()),
        " ",
    ))
}

/// Decodes the ciphertext using the affine cipher with key (`a`, `b`). Note that, rather than
/// returning a return code, the more common convention in Rust is to return a `Result`.
pub fn decode(ciphertext: &str, a: i32, b: i32) -> Result<String, AffineCipherError> {
    if egcd(a, M).0 != 1 {
        return Err(AffineCipherError::NotCoprime(a));
    }

    modinverse(a, M).map_or(Err(AffineCipherError::NotCoprime(a)), |a_inv| {
        Ok(ciphertext
            .chars()
            .filter_map(|c| match c {
                c if char::is_alphabetic(c) => decode_impl(c, a_inv, b).into(),
                c if char::is_numeric(c) => c.into(),
                _ => None,
            })
            .collect::<String>())
    })
}

fn encode_impl(c: char, a: i32, b: i32) -> char {
    let x = alphabet_to_index(c.to_ascii_lowercase());
    let n = (a * x + b) % M;
    index_to_alphabet(n)
}

fn decode_impl(c: char, a_inv: i32, b: i32) -> char {
    let x = alphabet_to_index(c.to_ascii_lowercase());
    let index = (a_inv * (x - b)).rem_euclid(M);
    index_to_alphabet(index)
}

fn alphabet_to_index(alphabet: char) -> i32 {
    ALPHABET.iter().position(|&c| c == alphabet).unwrap() as i32
}

fn index_to_alphabet(index: i32) -> char {
    ALPHABET[index as usize]
}
