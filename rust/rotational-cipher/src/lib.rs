pub fn rotate(letters: &str, n: u8) -> String {
    letters
        .chars()
        .map(|c|
            match (c.is_ascii_alphabetic(), c.is_ascii_uppercase()) {
                (true, true)  => rot(c, 'A', n),
                (true, false) => rot(c, 'a', n),
                _             => c
            }
        )
        .collect()
}

fn rot(letter: char, base_letter: char, n: u8) -> char {
    let code_point = letter as u8;
    let base_code_point = base_letter as u8;
    let rot_code_point = base_code_point + (code_point - base_code_point + n % 26) % 26;
    rot_code_point as char
}
