//! GRIN RC capacitive-touch model.
//!
//! The upstream QMK firmware samples nine digital touch inputs:
//! left:  PA13, PA10, PA9, PA8
//! right: PC14, PC13, PC15, PA2
//! center: PA1
//!
//! The four-point sliders are interpreted as positions 100/75/50/25.

#![allow(dead_code)]

pub const LEFT_PINS: [&str; 4] = ["PA13", "PA10", "PA9", "PA8"];
pub const RIGHT_PINS: [&str; 4] = ["PC14", "PC13", "PC15", "PA2"];
pub const CENTER_PIN: &str = "PA1";

pub const POSITION_WEIGHTS: [i16; 4] = [100, 75, 50, 25];

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TouchEvent {
    LeftTap,
    RightTap,
    CenterTap,
    CenterLongPress,
    LeftSlide { position: i16, delta: i16 },
    RightSlide { position: i16, delta: i16 },
}

pub fn weighted_position(active: [bool; 4]) -> Option<i16> {
    let mut count = 0i16;
    let mut total = 0i16;

    for (pressed, weight) in active.into_iter().zip(POSITION_WEIGHTS) {
        if pressed {
            count += 1;
            total += weight;
        }
    }

    if count == 0 {
        None
    } else {
        Some(total / count)
    }
}
