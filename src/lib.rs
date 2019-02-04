#![warn(missing_debug_implementations)]
pub mod api_types;
pub mod normalized_types;
pub mod validators;

pub fn decode_base_36(digits: &str) -> u64 {
    let mut value = 0;

    for digit in digits.chars() {
        let digit_value = match digit {
            '0'..='9' => u32::from(digit) - u32::from('0'),
            'a'..='z' => 10 + u32::from(digit) - u32::from('a'),
            _ => panic!("non-base-36 digit!"),
        };

        value = (value * 36) + u64::from(digit_value);
    }

    value
}
