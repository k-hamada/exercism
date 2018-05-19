#[derive(Debug, PartialEq)]
pub enum Error {
    SpanTooLong,
    InvalidDigit(char),
}

pub fn lsp(string_digits: &str, span: usize) -> Result<u64, Error> {
    if span == 0 { return Ok(1) }
    if string_digits.len() < span { return Err(Error::SpanTooLong) }

    string_digits.chars()
        .map(|c| c.to_digit(10).map(|n| n as u64).ok_or(c) )
        .collect::<Result<Vec<_>, _>>()
        .map_err(|c| Error::InvalidDigit(c) )
        .map(|digits|
            digits.windows(span)
                .map(|window| window.iter().product() )
                .max()
                .unwrap()
        )
}
