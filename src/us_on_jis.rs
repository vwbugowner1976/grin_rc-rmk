//! US-on-JIS behavioral contract for GRIN RC.
//!
//! HID usages are interpreted through the host keyboard layout. This module
//! records the intended US punctuation usages without pretending that a USB
//! usage alone guarantees a Unicode character on every Japanese OS.

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UsOnJisKey {
    LeftBracket, RightBracket, Backslash, Grave, Minus, Equal,
    Semicolon, Quote, Comma, Dot, Slash,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Translation { pub key: UsOnJisKey, pub usage: u8 }

pub const US_TRANSLATIONS: [Translation; 11] = [
    Translation { key: UsOnJisKey::LeftBracket, usage: 0x2F },
    Translation { key: UsOnJisKey::RightBracket, usage: 0x30 },
    Translation { key: UsOnJisKey::Backslash, usage: 0x31 },
    Translation { key: UsOnJisKey::Grave, usage: 0x35 },
    Translation { key: UsOnJisKey::Minus, usage: 0x2D },
    Translation { key: UsOnJisKey::Equal, usage: 0x2E },
    Translation { key: UsOnJisKey::Semicolon, usage: 0x33 },
    Translation { key: UsOnJisKey::Quote, usage: 0x34 },
    Translation { key: UsOnJisKey::Comma, usage: 0x36 },
    Translation { key: UsOnJisKey::Dot, usage: 0x37 },
    Translation { key: UsOnJisKey::Slash, usage: 0x38 },
];

pub const fn usage_for(key: UsOnJisKey) -> u8 {
    match key {
        UsOnJisKey::LeftBracket => 0x2F, UsOnJisKey::RightBracket => 0x30,
        UsOnJisKey::Backslash => 0x31, UsOnJisKey::Grave => 0x35,
        UsOnJisKey::Minus => 0x2D, UsOnJisKey::Equal => 0x2E,
        UsOnJisKey::Semicolon => 0x33, UsOnJisKey::Quote => 0x34,
        UsOnJisKey::Comma => 0x36, UsOnJisKey::Dot => 0x37,
        UsOnJisKey::Slash => 0x38,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn bracket_usages_are_us_hid_usages() {
        assert_eq!(usage_for(UsOnJisKey::LeftBracket), 0x2F);
        assert_eq!(usage_for(UsOnJisKey::RightBracket), 0x30);
        assert_eq!(usage_for(UsOnJisKey::Backslash), 0x31);
    }
}
