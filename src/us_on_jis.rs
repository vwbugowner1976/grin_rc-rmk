//! US-layout-on-JIS-host compatibility layer.
//!
//! The original helper supplied for this project remaps punctuation so a host
//! configured for a Japanese keyboard still produces US-layout symbols.
//! This module defines the translation surface; HID-event integration follows
//! after the base Rynk path builds on hardware.

#![allow(dead_code)]

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UsOnJisKey {
    Kc2,
    Kc6,
    Kc7,
    Kc8,
    Kc9,
    Kc0,
    Minus,
    Equal,
    LeftBracket,
    RightBracket,
    Backslash,
    Semicolon,
    Quote,
    Grave,
}
