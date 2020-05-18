use std::cmp::max;
use std::cmp::Ordering;
use std::ops::Add;
use std::ops::Mul;
use std::ops::Sub;

extern crate num_bigint;
use num_bigint::BigUint;
use num_bigint::Sign;
use num_traits::Num;
use num_traits::Pow;
use num_traits::Zero;

impl Add for Decimal {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        if self == Self::zero() {
            return other;
        }

        if other == Self::zero() {
            return self;
        }

        let (sign, fraction, exponent) = {
            let eq_sign = self.sign == other.sign;
            let cmp_exponent = self.exponent.cmp(&other.exponent);
            let (self_fraction, other_fraction) = match cmp_exponent {
                Ordering::Equal => (self.fraction, other.fraction),
                Ordering::Greater => (self.fraction, other.fraction_expand(self.exponent)),
                Ordering::Less => (self.fraction_expand(other.exponent), other.fraction),
            };
            let cmp_fraction = self_fraction.cmp(&other_fraction);

            match (eq_sign, cmp_exponent, cmp_fraction) {
                (true, Ordering::Equal, _) => {
                    (self.sign, self_fraction + other_fraction, self.exponent)
                }
                (true, Ordering::Greater, _) => {
                    (self.sign, self_fraction + other_fraction, self.exponent)
                }
                (true, Ordering::Less, _) => {
                    (self.sign, self_fraction + other_fraction, other.exponent)
                }
                (false, Ordering::Equal, Ordering::Greater) => {
                    (self.sign, self_fraction - other_fraction, self.exponent)
                }
                (false, Ordering::Equal, _) => {
                    (other.sign, other_fraction - self_fraction, self.exponent)
                }
                (false, Ordering::Greater, Ordering::Greater) => {
                    (self.sign, self_fraction - other_fraction, self.exponent)
                }
                (false, Ordering::Greater, _) => {
                    (self.sign, other_fraction - self_fraction, self.exponent)
                }
                (false, Ordering::Less, Ordering::Greater) => {
                    (self.sign, self_fraction - other_fraction, other.exponent)
                }
                (false, Ordering::Less, _) => {
                    (self.sign, self_fraction - other_fraction, other.exponent)
                }
            }
        };

        Self::create(sign, fraction, exponent)
    }
}

impl Sub for Decimal {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        self + other.negate()
    }
}

impl Mul for Decimal {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        let sign = if self.sign == other.sign {
            Sign::Plus
        } else {
            Sign::Minus
        };

        Self::create(sign, self.fraction * other.fraction, self.exponent + other.exponent)
    }
}

impl PartialEq for Decimal {
    fn eq(&self, other: &Self) -> bool {
        let s_decimal = self.reduce();
        let o_decimal = other.reduce();

        s_decimal.sign == o_decimal.sign
            && s_decimal.fraction == o_decimal.fraction
            && s_decimal.exponent == o_decimal.exponent
    }
}

impl PartialOrd for Decimal {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self.sign, other.sign) {
            (Sign::Plus, Sign::Minus) | (Sign::Plus, Sign::NoSign) => Ordering::Greater.into(),
            (Sign::Minus, Sign::Plus) | (Sign::Minus, Sign::NoSign) => Ordering::Less.into(),
            (Sign::Plus, Sign::Plus) => {
                let max_exponent = max(self.exponent, other.exponent);
                if self.fraction_expand(max_exponent) > other.fraction_expand(max_exponent) {
                    Ordering::Greater.into()
                } else {
                    Ordering::Less.into()
                }
            }
            (Sign::Minus, Sign::Minus) => {
                let max_exponent = max(self.exponent, other.exponent);
                if self.fraction_expand(max_exponent) > other.fraction_expand(max_exponent) {
                    Ordering::Less.into()
                } else {
                    Ordering::Greater.into()
                }
            }
            (Sign::NoSign, Sign::Minus) => Ordering::Greater.into(),
            (Sign::NoSign, Sign::Plus) => Ordering::Less.into(),
            (Sign::NoSign, Sign::NoSign) => Ordering::Equal.into(),
        }
    }
}

#[derive(Eq, Debug)]
pub struct Decimal {
    sign: Sign,
    fraction: BigUint,
    exponent: usize,
}

impl Decimal {
    pub fn try_from(input: &str) -> Option<Decimal> {
        let num = &input.replace(|c| c == '-' || c == '.', "");
        if let Ok(fraction) = BigUint::from_str_radix(num, 10) {
            let sign = if input.starts_with('-') {
                Sign::Minus
            } else {
                Sign::Plus
            };

            let exponent = input.rfind('.').map(|p| input.len() - 1 - p).unwrap_or(0);

            return Self::create(sign, fraction, exponent).into();
        }

        None
    }

    fn create(sign: Sign, fraction: BigUint, exponent: usize) -> Self {
        if fraction == BigUint::zero() && exponent == 0 {
            return Self::zero();
        }

        Self {
            sign,
            fraction,
            exponent,
        }
    }

    fn zero() -> Self {
        Self {
            sign: Sign::NoSign,
            fraction: BigUint::zero(),
            exponent: 0,
        }
    }

    fn reduce(&self) -> Self {
        let mut fraction = self.fraction.clone();
        let mut exponent = self.exponent;
        let v = 10_u8;

        while &fraction % v == BigUint::zero() && exponent > 0 {
            fraction /= v;
            exponent -= 1;
        }

        Self::create(self.sign, fraction, exponent)
    }

    fn negate(&self) -> Self {
        let sign = match self.sign {
            Sign::Plus => Sign::Minus,
            Sign::Minus => Sign::Plus,
            Sign::NoSign => Sign::NoSign,
        };

        Self::create(sign, self.fraction.clone(), self.exponent)
    }

    fn fraction_expand(&self, max_exponent: usize) -> BigUint {
        let n: BigUint = 10_u32.into();
        let coefficient = n.pow(max_exponent - self.exponent);
        &self.fraction * &coefficient
    }
}
