pub struct Luhn(Vec<u32>);

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

pub trait ValidLuhn {
    fn valid_luhn(self) -> bool;
}

impl ValidLuhn for String {
    fn valid_luhn(self) -> bool {
        Luhn::from(self).is_valid()
    }
}

impl ValidLuhn for &'static str {
    fn valid_luhn(self) -> bool {
        Luhn::from(self.to_string()).is_valid()
    }
}

impl ValidLuhn for usize {
    fn valid_luhn(self) -> bool {
        Luhn::from(self.to_string()).is_valid()
    }
}

impl ValidLuhn for u8 {
    fn valid_luhn(self) -> bool {
        Luhn::from(self.to_string()).is_valid()
    }
}

impl ValidLuhn for u16 {
    fn valid_luhn(self) -> bool {
        Luhn::from(self.to_string()).is_valid()
    }
}

impl ValidLuhn for u32 {
    fn valid_luhn(self) -> bool {
        Luhn::from(self.to_string()).is_valid()
    }
}

impl ValidLuhn for u64 {
    fn valid_luhn(self) -> bool {
        Luhn::from(self.to_string()).is_valid()
    }
}
