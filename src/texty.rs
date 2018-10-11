use sha2::Digest;
use std::{convert::TryFrom, slice::SliceConcatExt};

type Hash = sha2::Sha512Trunc256;

const TERM_BG_BLACK: &str = "\x1b[40m";
const TERM_RESET: &str = "\x1b[0m";
const TERM_FG_COLORS: &[&str] = &[
    "\x1b[91m", "\x1b[92m", "\x1b[93m", "\x1b[94m", "\x1b[95m", "\x1b[96m", "\x1b[31m", "\x1b[32m",
    "\x1b[33m", "\x1b[34m", "\x1b[35m", "\x1b[36m",
];

pub fn color_with_hash(string: &str) -> String {
    let digest = Hash::digest(string.as_bytes());
    let u = (usize::from(digest[0]) << 0) + (usize::from(digest[1]) << 1);

    let bg = TERM_BG_BLACK;
    let fg = TERM_FG_COLORS[u % TERM_FG_COLORS.len()];

    [TERM_RESET, bg, fg, string, TERM_RESET].join("")
}

pub fn country_flag(country_code: &str) -> String {
    country_code
        .chars()
        .filter(|c| c.is_ascii_alphabetic())
        .take(2)
        .map(|letter| {
            let letter_scalar: u32 = letter.to_ascii_uppercase().into();
            let flag_sclar = 0x1F1A5 + letter_scalar;
            char::try_from(flag_sclar).expect("this must be a valid code point")
        })
        .collect()
}
