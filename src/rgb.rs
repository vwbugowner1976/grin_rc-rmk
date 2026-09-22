//! GRIN RC RGB matrix contract.
//!
//! The upstream QMK firmware has 66 WS2812-compatible LEDs on PA0.

pub const DATA_PIN: &str = "PA0";
pub const LED_COUNT: usize = 66;
pub const MAX_BRIGHTNESS: u8 = 200;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SliderMode { None, Rgb, Volume, Scroll, Party }

pub const fn rgb_value_for_slider(value: i16) -> u8 {
    match value { 0..=25 => 0, 26..=49 => 33, 50 => 67, 51..=74 => 100, 75 => 133, 76..=99 => 167, _ => 200 }
}
pub const fn hue_for_slider(value: i16) -> u16 {
    match value { 0..=25 => 180, 26..=49 => 150, 50 => 120, 51..=74 => 90, 75 => 60, 76..=99 => 30, _ => 0 }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn brightness_matches_qmk_slider_buckets() {
        assert_eq!(rgb_value_for_slider(25), 0);
        assert_eq!(rgb_value_for_slider(50), 67);
        assert_eq!(rgb_value_for_slider(75), 133);
        assert_eq!(rgb_value_for_slider(100), 200);
    }
    #[test]
    fn hue_matches_qmk_slider_buckets() {
        assert_eq!(hue_for_slider(25), 180);
        assert_eq!(hue_for_slider(50), 120);
        assert_eq!(hue_for_slider(75), 60);
        assert_eq!(hue_for_slider(100), 0);
    }
}
