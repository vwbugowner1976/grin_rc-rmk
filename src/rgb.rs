//! RGB parity target for the upstream GRIN RC QMK firmware.
//!
//! Hardware: 66 x WS2812-compatible LEDs, data on PA0.
//! Full QMK RGB Matrix effect parity is intentionally tracked separately from
//! RMK's core keyboard bring-up because it needs a dedicated async LED task.

#![allow(dead_code)]

pub const DATA_PIN: &str = "PA0";
pub const LED_COUNT: usize = 66;
pub const MAX_BRIGHTNESS: u8 = 200;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SliderMode {
    None,
    Rgb,
    Volume,
    Scroll,
    Party,
}
