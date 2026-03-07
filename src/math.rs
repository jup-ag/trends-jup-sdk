use crate::QuoteError;

pub(crate) fn checked_add(a: u64, b: u64) -> Result<u64, QuoteError> {
    a.checked_add(b).ok_or(QuoteError::MathOverflow)
}

pub(crate) fn checked_sub(a: u64, b: u64) -> Result<u64, QuoteError> {
    a.checked_sub(b).ok_or(QuoteError::MathUnderflow)
}

pub(crate) fn checked_mul_u128(a: u128, b: u128) -> Result<u128, QuoteError> {
    a.checked_mul(b).ok_or(QuoteError::MathOverflow)
}

pub(crate) fn checked_add_u128(a: u128, b: u128) -> Result<u128, QuoteError> {
    a.checked_add(b).ok_or(QuoteError::MathOverflow)
}

pub(crate) fn try_u64(value: u128) -> Result<u64, QuoteError> {
    value
        .try_into()
        .map_err(|_| QuoteError::IntegerConversionOverflow)
}
