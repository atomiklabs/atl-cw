use std::ops::Sub;

use crate::signed_decimal::SignedDecimal;

impl Sub for SignedDecimal {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            // x - y => x - y >= 0  if x >= y || x - y < 0 if y > x
            (Self::NonNegative(lhs), Self::NonNegative(rhs)) => {
                if lhs >= rhs {
                    Self::NonNegative(lhs - rhs)
                } else {
                    Self::Negative(rhs - lhs)
                }
            }

            // -x - y < 0
            (Self::Negative(lhs), Self::NonNegative(rhs)) => Self::Negative(lhs + rhs),

            // x - (-y) = x + y > 0
            (Self::NonNegative(lhs), Self::Negative(rhs)) => Self::NonNegative(lhs + rhs),

            // -x - (-y) = -x + y => y - x >= 0 if y >= x || y - x < 0 if x > y
            (Self::Negative(lhs), Self::Negative(rhs)) => {
                if rhs >= lhs {
                    Self::NonNegative(rhs - lhs)
                } else {
                    Self::Negative(lhs - rhs)
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use cosmwasm_std::Decimal;

    use crate::signed_decimal::SignedDecimal;

    type SD = SignedDecimal<Decimal>;

    #[test]
    fn can_sub_two_decimals_when_both_non_negative() {
        // x - y where |x| > |y|
        assert_eq!(
            SD::from_str("5.73").unwrap() - SD::from_str("5.71").unwrap(),
            SD::from_str("0.02").unwrap()
        );

        // x - y where |x| = |y|
        assert_eq!(
            SD::from_str("5.73").unwrap() - SD::from_str("5.73").unwrap(),
            SD::zero()
        );

        // x - y where |x| < |y|
        assert_eq!(
            SD::from_str("5.71").unwrap() - SD::from_str("5.73").unwrap(),
            SD::from_str("-0.02").unwrap()
        );
    }

    #[test]
    fn can_sub_two_decimals_when_one_non_negative_and_one_negative() {
        // x - -y = x + y where |x| > |y|
        assert_eq!(
            SD::from_str("5.73").unwrap() - SD::from_str("-5.71").unwrap(),
            SD::from_str("11.44").unwrap()
        );

        // x - -y where |x| = |y|
        assert_eq!(
            SD::from_str("5.73").unwrap() - SD::from_str("-5.73").unwrap(),
            SD::from_str("11.46").unwrap()
        );

        // x - -y = x + y where |x| < |y|

        assert_eq!(
            SD::from_str("5.71").unwrap() - SD::from_str("-5.73").unwrap(),
            SD::from_str("11.44").unwrap()
        );

        // -x - y where |x| > |y|
        assert_eq!(
            SD::from_str("-5.73").unwrap() - SD::from_str("5.71").unwrap(),
            SD::from_str("-11.44").unwrap()
        );

        // -x - y where |x| = |y|
        assert_eq!(
            SD::from_str("-5.73").unwrap() - SD::from_str("5.73").unwrap(),
            SD::from_str("-11.46").unwrap()
        );

        // -x + y where |x| < |y|
        assert_eq!(
            SD::from_str("-5.71").unwrap() - SD::from_str("5.73").unwrap(),
            SD::from_str("-11.44").unwrap()
        );
    }

    #[test]
    fn can_sub_two_decimals_when_both_negative() {
        // -x - (-y) = -x + y = y - x when |x| > |y|
        assert_eq!(
            SD::from_str("-5.73").unwrap() - SD::from_str("-5.71").unwrap(),
            SD::from_str("-0.02").unwrap()
        );

        // -x - (-y) = -x + y = y - x when |x| = |y|
        assert_eq!(
            SD::from_str("-5.73").unwrap() - SD::from_str("-5.73").unwrap(),
            SD::zero()
        );

        // -x - (-y) = -x + y = y - x when |x| < |y|
        assert_eq!(
            SD::from_str("-5.71").unwrap() - SD::from_str("-5.73").unwrap(),
            SD::from_str("0.02").unwrap()
        );
    }
}
