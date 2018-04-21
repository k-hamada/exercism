/// Determines whether the supplied string is a valid ISBN number
pub fn is_valid_isbn(isbn: &str) -> bool {
    let numbers = isbn.chars().flat_map(|c| parse(c)).collect::<Vec<_>>();
    if numbers.iter().rev().skip(1).any(|&n| n == 10) {
        return false;
    }
    (1..11).rev().zip(numbers).map(|(i, n)| i * n).sum::<u32>() % 11 == 0
}

fn parse(number: char) -> Option<u32> {
    if number.is_digit(10) {
        return number.to_digit(10);
    }
    if number.eq_ignore_ascii_case(&'x') {
        return Some(10);
    }
    None
}

#[test]
fn test_parse_1() {
    assert_eq!(Some(1), parse('1'));
}

#[test]
fn test_parse_10() {
    assert_eq!(Some(10), parse('x'));
    assert_eq!(Some(10), parse('X'));
}

#[test]
fn test_parse_err() {
    assert_eq!(None, parse('a'));
}
