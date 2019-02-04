use crate::data::types::Id64;

use derive_more::From;
use err_derive::Error;

/// Errors for [id64_from_base36].
#[derive(Debug, Error, From)]
pub enum Base36DecodingError {
    #[error(display = "invalid digit: {:?}", _0)]
    InvalidDigit(char),
    #[error(display = "value was zero")]
    Zero,
}

/// Decodes a nonzero lowercase base 36 string to an [Id64].
pub fn id64_from_base36(digits: &str) -> Result<Id64, Base36DecodingError> {
    let mut value = 0;

    for digit in digits.chars() {
        let digit_value = match digit {
            '0'..='9' => u32::from(digit) - u32::from('0'),
            'a'..='z' => 10 + u32::from(digit) - u32::from('a'),
            _ => return Err(Base36DecodingError::InvalidDigit(digit)),
        };

        value = (value * 36) + u64::from(digit_value);
    }

    Id64::new(value)
        .map(Ok)
        .unwrap_or(Err(Base36DecodingError::Zero))
}

/// Encodes a u64 value to a lowercase base 36 string.
pub fn base36(value: impl Into<u64>) -> String {
    let mut digits: Vec<u8> = vec![];

    let mut value = value.into();
    while value > 0 {
        let digit = (value % 36) as usize;
        value /= 36;

        digits.push(b"012346789abcdefghijklmnopqrstuvwxyz"[digit]);
    }

    String::from_utf8(digits).unwrap()
}
