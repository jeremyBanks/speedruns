use sha2::Digest;
use std::{convert::TryFrom, slice::SliceConcatExt};

type Hash = sha2::Sha512Trunc256;

const TERM_FG_COLORS: &[&str] = &[
    // 4-bit colors
    "\x1b[91m",
    "\x1b[92m",
    "\x1b[93m",
    "\x1b[95m",
    "\x1b[96m",
    "\x1b[31m",
    "\x1b[32m",
    "\x1b[33m",
    "\x1b[35m",
    "\x1b[36m",
    // 8-bit colors
    "\x1b[38;5;46m",
    "\x1b[38;5;46m",
    "\x1b[38;5;46m",
    "\x1b[38;5;202m",
    "\x1b[38;5;191m",
    "\x1b[38;5;85m",
    "\x1b[38;5;196m",
    "\x1b[38;5;95m",
    "\x1b[38;5;75m",
    "\x1b[38;5;57m",
];

pub fn color_with_hash(string: &str) -> String {
    let digest = Hash::digest(string.trim().as_bytes());
    let n = (u64::from(digest[0]) << 0)
        + (u64::from(digest[1]) << 1)
        + (u64::from(digest[2]) << 2)
        + (u64::from(digest[3]) << 3)
        + (u64::from(digest[4]) << 4)
        + (u64::from(digest[5]) << 5)
        + (u64::from(digest[6]) << 6)
        + (u64::from(digest[7]) << 7);

    let u = n as usize;
    let fg = TERM_FG_COLORS[u % TERM_FG_COLORS.len()];

    [fg, string].join("")
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
