#[derive(Debug, PartialEq)]
pub enum Error {
    IncompleteNumber,
    Overflow,
}

/// Convert a list of numbers to a stream of bytes encoded with variable length encoding.
pub fn to_bytes(values: &[u32]) -> Vec<u8> {
    let mut res = vec![];
    for value in values {
        res.append(&mut to_byte(value));
    }
    res
}

fn to_byte(value: &u32) -> Vec<u8> {
    let mut res = vec![];
    res.push((value & 0x7F) as u8);
    let mut val: u32 = value >> 7;
    while val > 0 {
        res.push((0x80 + (val & 0x7f)) as u8);
        val >>= 7
    }
    res.reverse();
    res
}

/// Given a stream of bytes, extract all numbers which are encoded in there.
pub fn from_bytes(bytes: &[u8]) -> Result<Vec<u32>, Error> {
    let mut res = vec![];
    let mut value = 0u32;
    for byte in bytes {
        value = value << 7 | (byte & 0x7F) as u32;
        if byte < &0x80 {
            res.push(value);
            value = 0u32;
        }

        if value > u32::max_value() >> 7 {
            return Err(Error::Overflow);
        }
    }

    if res.is_empty() {
        return Err(Error::IncompleteNumber);
    }

    Ok(res)
}
