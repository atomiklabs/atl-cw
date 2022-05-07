pub trait CwDecimal: Sized {
    type CwInt;

    fn is_zero(&self) -> bool;

    fn sqrt(&self) -> Self;

    fn percent(x: u64) -> Self;

    fn permille(x: u64) -> Self;

    fn from_ratio(numerator: impl Into<Self::CwInt>, denominator: impl Into<Self::CwInt>) -> Self;
}
