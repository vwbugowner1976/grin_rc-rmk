//! GRIN RC touch gesture state machine.
//!
//! Mirrors the timing/state rules in the upstream QMK firmware.
//! GPIO sampling stays outside this pure state machine so behavior is testable.

pub const LEFT_PINS: [&str; 4] = ["PA13", "PA10", "PA9", "PA8"];
pub const RIGHT_PINS: [&str; 4] = ["PC14", "PC13", "PC15", "PA2"];
pub const CENTER_PIN: &str = "PA1";
pub const POSITION_WEIGHTS: [i16; 4] = [100, 75, 50, 25];
pub const SLIDER_DEBOUNCE: u16 = 400;
pub const CENTER_DEBOUNCE: u16 = 300;
pub const TAP_MAX_MS: u32 = 300;
pub const SLIDE_START_MS: u32 = 500;
pub const CENTER_LONG_MS: u32 = 800;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SliderMode { None, Rgb, Volume, Scroll, Party }

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TouchEvent {
    LeftTap, RightTap, CenterTap, CenterLongPress,
    LeftSlide { position: i16, delta: i16, elapsed_ms: u32 },
    RightSlide { position: i16, delta: i16, elapsed_ms: u32 },
}

pub fn weighted_position(active: [bool; 4]) -> Option<i16> {
    let mut count = 0i16;
    let mut total = 0i16;
    for (pressed, weight) in active.into_iter().zip(POSITION_WEIGHTS) {
        if pressed { count += 1; total += weight; }
    }
    (count != 0).then_some(total / count)
}

#[derive(Clone, Copy, Debug)]
struct Electrode { count: u16, active: bool }
impl Electrode {
    const fn new() -> Self { Self { count: 0, active: false } }
    fn update(&mut self, touched: bool, threshold: u16) -> bool {
        if touched { self.count = self.count.saturating_add(1).min(threshold.saturating_mul(2)); }
        else { self.count = self.count.saturating_sub(1); }
        let next = self.count > threshold;
        let changed = next != self.active;
        self.active = next;
        changed
    }
}

#[derive(Clone, Copy, Debug)]
pub struct SliderState {
    electrodes: [Electrode; 4],
    start_ms: Option<u32>,
    last_ms: u32,
    last_value: i16,
    tapped: bool,
}
impl SliderState {
    pub const fn new() -> Self {
        Self { electrodes: [Electrode::new(); 4], start_ms: None, last_ms: 0, last_value: 0, tapped: false }
    }
    pub fn update(&mut self, raw: [bool; 4], now_ms: u32, left: bool) -> Option<TouchEvent> {
        let mut changed = false;
        for i in 0..4 { changed |= self.electrodes[i].update(raw[i], SLIDER_DEBOUNCE); }
        let active = self.active();
        let value = weighted_position(active).unwrap_or(0);
        if changed {
            match (self.start_ms, active != [false; 4]) {
                (None, true) => {
                    self.start_ms = Some(now_ms); self.last_ms = now_ms;
                    self.last_value = value; self.tapped = false;
                }
                (Some(_), false) => {
                    if now_ms.wrapping_sub(self.start_ms.unwrap()) <= TAP_MAX_MS { self.tapped = true; }
                    self.last_ms = now_ms;
                }
                (Some(start), true) => {
                    if now_ms.wrapping_sub(start) > SLIDE_START_MS {
                        let delta = value - self.last_value;
                        self.last_value = value; self.last_ms = now_ms; self.tapped = false;
                        return Some(if left {
                            TouchEvent::LeftSlide { position: value, delta, elapsed_ms: now_ms.wrapping_sub(start) }
                        } else {
                            TouchEvent::RightSlide { position: value, delta, elapsed_ms: now_ms.wrapping_sub(start) }
                        });
                    }
                }
                _ => {}
            }
        } else if let Some(start) = self.start_ms {
            if active != [false; 4] {
                if !self.tapped && now_ms.wrapping_sub(self.last_ms) > TAP_MAX_MS {
                    self.last_ms = now_ms;
                    let delta = value - self.last_value;
                    self.last_value = value;
                    return Some(if left {
                        TouchEvent::LeftSlide { position: value, delta, elapsed_ms: now_ms.wrapping_sub(start) }
                    } else {
                        TouchEvent::RightSlide { position: value, delta, elapsed_ms: now_ms.wrapping_sub(start) }
                    });
                }
            } else if now_ms.wrapping_sub(self.last_ms) > TAP_MAX_MS {
                let was_tap = self.tapped;
                self.start_ms = None; self.last_value = 0; self.tapped = false;
                if was_tap { return Some(if left { TouchEvent::LeftTap } else { TouchEvent::RightTap }); }
            }
        }
        None
    }
    fn active(&self) -> [bool; 4] {
        [self.electrodes[0].active, self.electrodes[1].active, self.electrodes[2].active, self.electrodes[3].active]
    }
}

#[derive(Clone, Copy, Debug)]
pub struct CenterState {
    count: u16,
    active: bool,
    start_ms: Option<u32>,
    long_sent: bool,
}
impl CenterState {
    pub const fn new() -> Self { Self { count: 0, active: false, start_ms: None, long_sent: false } }
    pub fn update(&mut self, raw_touched: bool, now_ms: u32) -> Option<TouchEvent> {
        if raw_touched { self.count = self.count.saturating_add(1).min(CENTER_DEBOUNCE.saturating_mul(2)); }
        else { self.count = self.count.saturating_sub(1); }
        let next = self.count > CENTER_DEBOUNCE;
        if next != self.active {
            self.active = next;
            if next {
                self.start_ms = Some(now_ms); self.long_sent = false;
            } else if let Some(start) = self.start_ms.take() {
                if !self.long_sent && now_ms.wrapping_sub(start) < CENTER_LONG_MS { return Some(TouchEvent::CenterTap); }
            }
        } else if self.active && !self.long_sent {
            if let Some(start) = self.start_ms {
                if now_ms.wrapping_sub(start) >= CENTER_LONG_MS {
                    self.long_sent = true;
                    return Some(TouchEvent::CenterLongPress);
                }
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn position_averages_multiple_electrodes() {
        assert_eq!(weighted_position([true, false, true, false]), Some(75));
        assert_eq!(weighted_position([false; 4]), None);
    }
    #[test]
    fn electrode_hysteresis_requires_more_than_threshold() {
        let mut e = Electrode::new();
        for _ in 0..400 { assert!(!e.update(true, 400)); }
        assert!(e.update(true, 400));
        assert!(e.active);
    }
}
