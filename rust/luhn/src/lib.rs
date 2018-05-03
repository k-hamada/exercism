/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    let codes: Option<Vec<_>> = code.chars()
        .map(|c| if c.is_ascii_digit() || c.is_whitespace() { Some(c) } else { None } )
        .collect();

    if codes.is_none() {
        return false;
    }

    let digits: Vec<_> = codes.unwrap()
        .iter()
        .flat_map(|c| c.to_digit(10) )
        .collect();

    if digits.len() <= 1 {
        return false;
    }

    let sum = digits
        .iter()
        .rev()
        .enumerate()
        .map(|(i, &n)| if i % 2 == 0 { n } else { n * 2 } )
        .map(|n| if n < 10 { n } else { n - 9 } )
        .sum::<u32>();
        
    sum % 10 == 0
}
