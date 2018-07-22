pub struct Luhn(Vec<u32>);

impl From<String> for Luhn {
    fn from(input: String) -> Self {
        input
            .chars()
            .filter(|c| !c.is_whitespace())
            .map(|c| {
                if c.is_ascii_digit() {
                    c.to_digit(10)
                } else {
                    None
                }
            })
            .collect::<Option<Vec<_>>>()
            .map_or_else(|| Luhn(Vec::new()), |v| Luhn(v))
    }
}

impl From<&'static str> for Luhn {
    fn from(input: &'static str) -> Self {
        Luhn::from(input.to_string())
    }
}
impl From<usize> for Luhn {
    fn from(input: usize) -> Self {
        Luhn::from(input.to_string())
    }
}
impl From<u8> for Luhn {
    fn from(input: u8) -> Self {
        Luhn::from(input.to_string())
    }
}
impl From<u16> for Luhn {
    fn from(input: u16) -> Self {
        Luhn::from(input.to_string())
    }
}
impl From<u32> for Luhn {
    fn from(input: u32) -> Self {
        Luhn::from(input.to_string())
    }
}
impl From<u64> for Luhn {
    fn from(input: u64) -> Self {
        Luhn::from(input.to_string())
    }
}

impl Luhn {
    pub fn is_valid(self) -> bool {
        let digits = self.0;

        if digits.len() <= 1 {
            return false;
        }

        let sum = digits
            .iter()
            .rev()
            .enumerate()
            .map(|(i, &n)| if i % 2 == 0 { n } else { n * 2 })
            .map(|n| if n < 10 { n } else { n - 9 })
            .sum::<u32>();

        sum % 10 == 0
    }
}
