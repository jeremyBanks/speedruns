#![allow(clippy::useless_attribute)]
#![warn(missing_debug_implementations)]

use derive_more::From;
use err_derive::Error;
#[allow(unused)]
use log::{debug, error, info, trace, warn};
/// Errors for [u64_from_base36].
#[derive(Debug, Error, From, PartialEq)]
pub enum Base36DecodingError {
    #[error(display = "invalid digit: {:?}", _0)]
    InvalidDigit(char),
    #[error(display = "value didn't have expected length of 8 characters")]
    WrongLength,
}

// Converts a name to a slug as speedrun.com would.
pub fn src_slugify(s: &str) -> String {
    let mut src_slug = String::new();

    // TODO: check against API URLs during normalization

    let mut last_was_spacing = true;
    for c in s.chars() {
        let mut this_was_spacing = false;
        match c {
            'a'..='z' | 'A'..='Z' | '0'..='9' | '+' | '-' => src_slug.push(c),
            '/' | '\'' => {}
            _ => {
                this_was_spacing = true;
                if !last_was_spacing {
                    src_slug.push('_');
                }
            }
        }
        last_was_spacing = this_was_spacing;
    }

    if last_was_spacing && !src_slug.is_empty() {
        src_slug.truncate(src_slug.len() - 1);
    }

    if src_slug.is_empty() {
        src_slug.push('_');
    }

    src_slug
}

/// Decodes a nonzero lowercase base 36 string to an [u64].
pub fn u64_from_base36(digits: &str) -> Result<u64, Base36DecodingError> {
    let mut value = 0;

    if digits.len() != 8 {
        return Err(Base36DecodingError::WrongLength);
    }

    for digit in digits.chars() {
        let digit_value = match digit {
            '0'..='9' => u32::from(digit) - u32::from('0'),
            'a'..='z' => 10 + u32::from(digit) - u32::from('a'),
            _ => return Err(Base36DecodingError::InvalidDigit(digit)),
        };

        value *= 36;
        value += u64::from(digit_value);
    }

    Ok(value)
}

/// Encodes a u64 value to a lowercase base 36 string.
pub fn base36(value: impl Into<u64>) -> String {
    let mut digits: Vec<u8> = vec![];

    let mut value = value.into();
    while value > 0 {
        let digit = (value % 36) as usize;
        value /= 36;

        digits.push(b"0123456789abcdefghijklmnopqrstuvwxyz"[digit]);
    }

    digits.reverse();
    format!("{:0>8}", String::from_utf8(digits).unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_base36() {
        assert_eq!(Err(Base36DecodingError::WrongLength), u64_from_base36(""));
        for (expected_id, expected_b36) in vec![
            (1u64, "00000001"),
            (35, "0000000z"),
            (35 * 36 * 36, "00000z00"),
            (36 * 36 * 36 * 36 * 36 * 36 * 36 * 36 - 1, "zzzzzzzz"),
        ] {
            let actual_id = u64_from_base36(expected_b36).unwrap();
            let actual_b36 = base36(expected_id);
            assert_eq!(expected_id, actual_id);
            assert_eq!(expected_b36, actual_b36);
        }
    }
}
