#[derive(Debug, PartialEq)]
pub enum Error {
    SpanTooLong,
    InvalidDigit(char),
}

pub fn lsp(string_digits: &str, span: usize) -> Result<u64, Error> {
    if span == 0 { return Ok(1) }
    if string_digits.len() < span { return Err(Error::SpanTooLong) }

    let digits = string_digits.chars()
        .map(|c| c.to_digit(10).map(|n| n as u64).ok_or(c) )
        .collect::<Result<Vec<_>, _>>();

    if digits.is_err() {
        return Err(Error::InvalidDigit(digits.unwrap_err()));
    };

    let max = digits.unwrap()
        .windows(span)
        .map(|window| window.iter().product() )
        .max();
    return Ok(max.unwrap())
}
