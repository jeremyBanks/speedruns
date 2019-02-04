use crate::data::types::Id64;

use derive_more::From;
use err_derive::Error;
#[allow(unused)] use log::{debug, error, info, trace, warn};
use unidecode::unidecode;
/// Errors for [id64_from_base36].
#[derive(Debug, Error, From, PartialEq)]
pub enum Base36DecodingError {
    #[error(display = "invalid digit: {:?}", _0)]
    InvalidDigit(char),
    #[error(display = "value was zero")]
    Zero,
    #[error(display = "value didn't have expected length of 8 characters")]
    WrongLength,
}

/// Converts a string into a canonical URL-safe slug containing only lowercase letters,
/// digits, and hyphen-dashes.
pub fn slugify(s: &str) -> String {
    let s = unidecode(s).to_ascii_lowercase();
    let mut slug = String::new();

    let mut last_was_replaced = false;
    for c in s.chars() {
        let mut this_was_replaced = false;
        match c {
            'a'..='z' | 'A'..='Z' | '0'..='9' => slug.push(c),
            '%' => slug.push_str("percent"),
            '+' => slug.push_str("plus"),
            '&' => slug.push_str("and"),
            '\'' => {}
            '/' => {
                this_was_replaced = true;
                if !last_was_replaced {
                    slug.push('-');
                }
                slug.push_str("or-")
            }
            _ => {
                this_was_replaced = true;
                if !last_was_replaced {
                    slug.push('-');
                }
            }
        }
        last_was_replaced = this_was_replaced;
    }

    if last_was_replaced {
        slug.truncate(slug.len() - 1);
    }

    slug
}

/// Decodes a nonzero lowercase base 36 string to an [Id64].
pub fn id64_from_base36(digits: &str) -> Result<Id64, Base36DecodingError> {
    let mut value = 0;

    if digits.len() != 8 {
        return Err(Base36DecodingError::WrongLength)
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

        digits.push(b"0123456789abcdefghijklmnopqrstuvwxyz"[digit]);
    }

    digits.reverse();
    format!("{:0>8}", String::from_utf8(digits).unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_slugify() {
        for (slug, name) in vec![
            ("celeste", "Celeste"),
            ("anypercent", "Any%"),
            ("100percent", "100%"),
            ("all-a-sides", "All A-Sides"),
            ("120-star", "120 Star"),
            ("new-game-pc", "New Game (PC)"),
            (
                "all-ng-memories-and-all-beads",
                "All NG Memories & All Beads",
            ),
            ("new-game-plus-pc", "New Game + (PC)"),
            ("all-red-berries", "All Red Berries"),
            ("resident-evil-2-2019", "Resident Evil 2 (2019)"),
            ("mickeys-speedway-usa", "Mickey's Speedway USA"),
            (
                "lego-star-wars-the-complete-saga-pc-or-console",
                "LEGO Star Wars: The Complete Saga (PC/Console)",
            ),
            ("mike-tysons-punch-out", "Mike Tyson's Punch-Out!!"),
            ("pokemon-blue", "Pokémon Blue"),
        ] {
            assert_eq!(slug, &slugify(name));
        }
    }

    #[test]
    fn test_base36() {
        assert_eq!(Err(Base36DecodingError::WrongLength), id64_from_base36(""));
        assert_eq!(Err(Base36DecodingError::Zero), id64_from_base36("00000000"));
        for (expected_id, expected_b36) in vec![
            (1u64, "00000001"),
            (35, "0000000z"),
            (35 * 36 * 36, "00000z00"),
            (36 * 36 * 36 * 36 * 36 * 36 * 36 * 36 - 1, "zzzzzzzz"),
        ] {
            let actual_id = u64::from(id64_from_base36(expected_b36).unwrap());
            let actual_b36 = base36(expected_id);
            assert_eq!(expected_id, actual_id);
            assert_eq!(expected_b36, actual_b36);
        }
    }
}
