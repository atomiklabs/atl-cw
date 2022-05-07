use std::{
    fmt::{Display, Write},
    iter::Sum,
    ops::Add,
    str::FromStr,
};

use cosmwasm_std::{Decimal, Uint128};

use crate::cw_decimal::CwDecimal;

#[derive(Clone, Debug, PartialEq)]
pub enum SignedDecimal<TDecimal = Decimal> {
    NonNegative(TDecimal),
    Negative(TDecimal),
}

impl<TDecimal> From<TDecimal> for SignedDecimal<TDecimal> {
    fn from(n: TDecimal) -> Self {
        Self::NonNegative(n)
    }
}

impl<TDecimal: FromStr> FromStr for SignedDecimal<TDecimal> {
    type Err = TDecimal::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let is_negative = s.starts_with('-');
        let mut s = s;

        if is_negative {
            // let's drop the minus sign
            s = &s[1..];
        }

        let decimal = TDecimal::from_str(s)?;

        if is_negative {
            return Ok(Self::Negative(decimal));
        }

        Ok(Self::NonNegative(decimal))
    }
}

impl Display for SignedDecimal<Decimal> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NonNegative(n) => n.fmt(f),
            Self::Negative(n) => {
                f.write_char('-')?;
                n.fmt(f)
            }
        }
    }
}

impl SignedDecimal<Decimal> {
    pub const fn zero() -> Self {
        Self::NonNegative(Decimal::zero())
    }

    pub const fn one() -> Self {
        Self::NonNegative(Decimal::one())
    }

    pub fn is_positive(&self) -> bool {
        if let Self::NonNegative(n) = self {
            !n.is_zero()
        } else {
            false
        }
    }

    pub fn is_negative(&self) -> bool {
        if let Self::Negative(_) = self {
            true
        } else {
            false
        }
    }
}

impl CwDecimal for SignedDecimal<Decimal> {
    type CwInt = Uint128;

    fn is_zero(&self) -> bool {
        match self {
            Self::NonNegative(n) => n.is_zero(),
            _ => false,
        }
    }

    fn sqrt(&self) -> Self {
        match self {
            Self::NonNegative(n) => Self::NonNegative(n.sqrt()),
            Self::Negative(n) => Self::Negative(n.sqrt()),
        }
    }

    fn percent(x: u64) -> Self {
        Self::NonNegative(Decimal::percent(x))
    }

    fn permille(x: u64) -> Self {
        Self::NonNegative(Decimal::permille(x))
    }

    fn from_ratio(numerator: impl Into<Self::CwInt>, denominator: impl Into<Self::CwInt>) -> Self {
        Self::NonNegative(Decimal::from_ratio(numerator, denominator))
    }
}

impl Sum for SignedDecimal<Decimal> {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::zero(), Add::add)
    }
}

#[cfg(test)]
mod tests {
    mod display {
        use std::str::FromStr;

        use crate::signed_decimal::SignedDecimal;

        #[test]
        fn can_display_non_negative_value() {
            assert_eq!(
                format!("{}", SignedDecimal::from_str("123.3123143901").unwrap()),
                "123.3123143901"
            );

            assert_eq!(format!("{}", SignedDecimal::zero()), "0");

            assert_eq!(format!("{}", SignedDecimal::one()), "1");
        }

        #[test]
        fn can_display_negative_value() {
            assert_eq!(
                format!("{}", SignedDecimal::from_str("-123.3123143901").unwrap()),
                "-123.3123143901"
            );
        }

        #[test]
        fn uses_to_string() {
            assert_eq!(
                SignedDecimal::from_str("-123.3123143901").unwrap().to_string(),
                "-123.3123143901"
            );
        }
    }

    mod aggregation {
        use std::{vec, str::FromStr};

        use crate::signed_decimal::SignedDecimal;

        #[test]
        fn can_calculate_sum() {
            let ledger = vec![
                SignedDecimal::from_str("-4.99").unwrap(),
                SignedDecimal::from_str("1.99").unwrap(),
                SignedDecimal::from_str("0.05399").unwrap(),
                SignedDecimal::from_str("476.190000010047").unwrap(),
            ];

            let sum: SignedDecimal = ledger.into_iter().sum();

            assert_eq!(SignedDecimal::from_str("473.243990010047").unwrap(), sum,);
        }
    }

    mod utils {
        use std::str::FromStr;

        use crate::signed_decimal::SignedDecimal;

        #[test]
        fn allows_checking_sign() {
            assert_eq!(
                SignedDecimal::from_str("-1").unwrap().is_negative(),
                true
            );

            assert_eq!(
                SignedDecimal::from_str("-1").unwrap().is_positive(),
                false
            );

            assert_eq!(
                SignedDecimal::from_str("1").unwrap().is_negative(),
                false
            );

            assert_eq!(
                SignedDecimal::from_str("1").unwrap().is_positive(),
                true
            );

            assert_eq!(
                SignedDecimal::from_str("0").unwrap().is_positive(),
                false
            );

            assert_eq!(
                SignedDecimal::from_str("0").unwrap().is_negative(),
                false
            );
        }
    }
}
