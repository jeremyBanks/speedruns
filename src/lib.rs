#![feature(associated_type_defaults)]
#![warn(missing_debug_implementations)]
#![allow(clippy::useless_attribute)]

pub mod api_types;
pub mod database;
pub mod normalize_api_types;
pub mod normalized_types;
pub mod validators;

pub use utils::*;

pub type BoxErr = Box<dyn std::error::Error>;

pub mod utils {
    use std::num::NonZeroU64 as id64;

    use derive_more::From;
    use err_derive::Error;

    #[derive(Debug, Error, From)]
    pub enum Error {
        #[error(display = "invalid digit: {:?}", _0)]
        InvalidDigit(char),
        #[error(display = "value was zero")]
        Zero,
    }

    pub fn id64_from_base36(digits: &str) -> Result<id64, Error> {
        let mut value = 0;

        for digit in digits.chars() {
            let digit_value = match digit {
                '0'..='9' => u32::from(digit) - u32::from('0'),
                'a'..='z' => 10 + u32::from(digit) - u32::from('a'),
                _ => return Err(Error::InvalidDigit(digit)),
            };

            value = (value * 36) + u64::from(digit_value);
        }

        id64::new(value).map(Ok).unwrap_or(Err(Error::Zero))
    }

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
}
