//! Shared utils.
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

    let mut last_was_spacing = true;
    let chars = s.chars().collect::<Vec<_>>();
    for i in 0..chars.len() {
        let mut this_was_spacing = false;
        let c = chars[i];
        let first_or_last = i == 0 || i == chars.len() - 1;
        match c {
            // unmodified
            'a'..='z' | '0'..='9' => slug.push(c),
            // always escaped
            '%' => slug.push_str("percent"),
            '+' => {
                if !last_was_spacing {
                    slug.push('-');
                }
                slug.push_str("plus-");
                this_was_spacing = true;
            }
            '&' => {
                if !last_was_spacing {
                    slug.push('-');
                }
                slug.push_str("and-");
                this_was_spacing = true;
            }
            '/' => {
                if !last_was_spacing {
                    slug.push('-');
                }
                slug.push_str("or-");
                this_was_spacing = true;
            }
            // escaped at ends, entirely ignored elsewhere
            '\'' =>
                if first_or_last {
                    if !last_was_spacing {
                        slug.push('-');
                    }
                    if first_or_last {
                        slug.push_str("prime-");
                    }
                    this_was_spacing = true;
                },
            // escaped at ends, converted to spacing elsewhere
            '.' => {
                if !last_was_spacing {
                    slug.push('-');
                }
                if first_or_last {
                    slug.push_str("dot-");
                }
                this_was_spacing = true;
            }
            '@' => {
                if !last_was_spacing {
                    slug.push('-');
                }
                if first_or_last {
                    slug.push_str("at-");
                }
                this_was_spacing = true;
            }
            '|' => {
                if !last_was_spacing {
                    slug.push('-');
                }
                if first_or_last {
                    slug.push_str("bar-");
                }
                this_was_spacing = true;
            }
            '_' => {
                if !last_was_spacing {
                    slug.push('-');
                }
                if first_or_last {
                    slug.push_str("underscore-");
                }
                this_was_spacing = true;
            }
            '-' => {
                if !last_was_spacing {
                    slug.push('-');
                }
                if first_or_last {
                    slug.push_str("minus-");
                }
                this_was_spacing = true;
            }
            // converted to spacing
            _ => {
                if !last_was_spacing {
                    slug.push('-');
                }
                this_was_spacing = true;
            }
        }
        last_was_spacing = this_was_spacing;
    }

    if last_was_spacing && !slug.is_empty() {
        slug.truncate(slug.len() - 1);
    }

    if slug.is_empty() {
        slug.push('-');
    }

    slug
}

// Converts a name to a slug as SpeedRun.Com would, for generating links.
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
        for (name, slug, src_slug) in vec![
            ("Celeste", "celeste", "Celeste"),
            ("Any%", "anypercent", "Any"),
            ("100%", "100percent", "100"),
            ("All A-Sides", "all-a-sides", "All_A-Sides"),
            ("120 Star", "120-star", "120_Star"),
            ("New Game (PC)", "new-game-pc", "New_Game_PC"),
            (
                "All NG Memories & All Beads",
                "all-ng-memories-and-all-beads",
                "All_NG_Memories_All_Beads",
            ),
            ("New Game + (PC)", "new-game-plus-pc", "New_Game_+_PC"),
            ("All Red Berries", "all-red-berries", "All_Red_Berries"),
            (
                "Resident Evil 2 (2019)",
                "resident-evil-2-2019",
                "Resident_Evil_2_2019",
            ),
            (
                "Mickey's Speedway USA",
                "mickeys-speedway-usa",
                "Mickeys_Speedway_USA",
            ),
            (
                "LEGO Star Wars: The Complete Saga (PC/Console)",
                "lego-star-wars-the-complete-saga-pc-or-console",
                "LEGO_Star_Wars_The_Complete_Saga_PCConsole",
            ),
            (
                "Mike Tyson's Punch-Out!!",
                "mike-tysons-punch-out",
                "Mike_Tysons_Punch-Out",
            ),
            ("Pokémon Blue", "pokemon-blue", "Pok_mon_Blue"),
            ("Route-Z'", "route-z-prime", "Route-Z"),
            ("Peace.", "peace-dot", "Peace"),
            (
                "Crash Bandicoot: N. Sane Trilogy",
                "crash-bandicoot-n-sane-trilogy",
                "Crash_Bandicoot_N_Sane_Trilogy",
            ),
            ("c-", "c-minus", "c-"),
        ] {
            assert_eq!(slug, &slugify(name));
            assert_eq!(src_slug, &src_slugify(name));
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
