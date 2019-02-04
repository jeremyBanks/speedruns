#![feature(associated_type_defaults)]
#![warn(missing_debug_implementations)]
pub mod api_types;
pub mod database;
pub mod normalize_api_types;
pub mod normalized_types;
pub mod validators;
pub use utils::*;

pub type DynError = Box<dyn std::error::Error>;

pub mod utils {
    use std::num::NonZeroU64 as p64;

    use derive_more::From;
    use err_derive::Error;

    #[derive(Debug, Error, From)]
    pub enum Error {
        #[error(display = "invalid digit: {:?}", _0)]
        InvalidDigit(char),
        #[error(display = "value was zero")]
        Zero,
    }

    pub fn p64_from_base36(digits: &str) -> Result<p64, Error> {
        let mut value = 0;

        for digit in digits.chars() {
            let digit_value = match digit {
                '0'..='9' => u32::from(digit) - u32::from('0'),
                'a'..='z' => 10 + u32::from(digit) - u32::from('a'),
                _ => return Err(Error::InvalidDigit(digit)),
            };

            value = (value * 36) + u64::from(digit_value);
        }

        p64::new(value).map(Ok).unwrap_or(Err(Error::Zero))
    }
}
