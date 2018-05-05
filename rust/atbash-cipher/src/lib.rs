/// "Encipher" with the Atbash cipher.
pub fn encode(plain: &str) -> String {
    plain
        .chars()
        .filter_map(|c| match c {
            '0'...'9' => Some(c),
            'A'...'z' => Some(atbash(c.to_ascii_lowercase())),
            _ => None,
        })
        .collect::<Vec<_>>()
        .chunks(5)
        .map(|window| window.iter().collect::<String>())
        .collect::<Vec<_>>()
        .join(" ")
}

/// "Decipher" with the Atbash cipher.
pub fn decode(cipher: &str) -> String {
    cipher
        .chars()
        .filter_map(|c| match c {
            '0'...'9' => Some(c),
            'a'...'z' => Some(atbash(c)),
            _ => None,
        })
        .collect()
}

fn atbash(c: char) -> char {
    ((25 - ((c as u8) - 97)) + 97) as char
}
