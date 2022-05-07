use std::str::FromStr;

use cosmwasm_std::{Decimal, Uint128};

use crate::cw_decimal::CwDecimal;

#[derive(Clone, Debug, PartialEq)]
pub enum SignedDecimal<TDecimal = Decimal> {
    NonNegative(TDecimal),
    Negative(TDecimal),
}

impl<TDecimal> From<TDecimal> for SignedDecimal<TDecimal>  {
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

impl SignedDecimal {
    pub const fn zero() -> Self {
        Self::NonNegative(Decimal::zero())
    }

    pub const fn one() -> Self {
        Self::NonNegative(Decimal::one())
    }
}

impl CwDecimal for SignedDecimal {
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
