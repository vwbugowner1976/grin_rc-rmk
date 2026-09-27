//! GRIN RC hardware controls: the three capacitive touch areas and WS2812 RGB.
//!
//! The original QMK firmware samples the touch electrodes as active-low inputs
//! with 400/300-count hysteresis and drives 66 WS2812 LEDs from PA0.  RMK 0.9
//! has a processor model, so the GRIN-specific hardware loop lives here instead
//! of being mixed into the matrix scanner.

use embassy_stm32::gpio::{Input, Output};
use embassy_time::{Duration, Instant};
use rmk::event::{publish_event, Axis, AxisEvent, AxisValType, LayerChangeEvent, PointingEvent};
use rmk::input_device::pointing::ALL_POINTING_DEVICES;
use rmk::macros::processor;

#[processor(subscribe = [LayerChangeEvent], poll_interval = 5)]
pub struct SingleTouchTest<'d> {
    left: [Input<'d>; 4],
    right: [Input<'d>; 4],
    center: Input<'d>,
}

impl<'d> SingleTouchTest<'d> {
    pub fn new(
        left: [Input<'d>; 4],
        right: [Input<'d>; 4],
        center: Input<'d>,
    ) -> Self {
        Self { left, right, center }
    }

    async fn on_layer_change_event(&mut self, _event: LayerChangeEvent) {}

    async fn poll(&mut self) {
        let left = [
            self.left[0].is_low(),
            self.left[1].is_low(),
            self.left[2].is_low(),
            self.left[3].is_low(),
        ];
        let right = [
            self.right[0].is_low(),
            self.right[1].is_low(),
            self.right[2].is_low(),
            self.right[3].is_low(),
        ];
        let _center = self.center.is_low();

        let _ = crate::touch::weighted_position(left);
        let _ = crate::touch::weighted_position(right);
    }
}


use crate::rgb::{hue_for_slider, rgb_value_for_slider};
use crate::touch::{CenterState, SliderMode, SliderState, TouchEvent};

const LED_COUNT: usize = 66;
const WS2812_T0H_CYCLES: u32 = 34; // ~0.35 us @ 96 MHz
const WS2812_T1H_CYCLES: u32 = 67; // ~0.70 us @ 96 MHz
const WS2812_BIT_CYCLES: u32 = 120; // ~1.25 us @ 96 MHz
const WS2812_RESET_CYCLES: u32 = 5_000; // ~52 us @ 96 MHz
#[derive(Clone, Copy)]
struct Hsv {
    h: u16,
    s: u8,
    v: u8,
}

impl Default for Hsv {
    fn default() -> Self {
        Self { h: 0, s: 255, v: 200 }
    }
}

#[processor(subscribe = [LayerChangeEvent], poll_interval = 5)]
pub struct GrinControl<'d> {
    left: [Input<'d>; 4],
    right: [Input<'d>; 4],
    center: Input<'d>,
    led: Output<'d>,
    left_state: SliderState,
    right_state: SliderState,
    center_state: CenterState,
    mode: SliderMode,
    hsv: Hsv,
    rgb_enabled: bool,
    last_party: Instant,
}

impl<'d> GrinControl<'d> {
    pub fn new(
        left: [Input<'d>; 4],
        right: [Input<'d>; 4],
        center: Input<'d>,
        led: Output<'d>,
    ) -> Self {
        Self {
            left,
            right,
            center,
            led,
            left_state: SliderState::new(),
            right_state: SliderState::new(),
            center_state: CenterState::new(),
            mode: SliderMode::None,
            hsv: Hsv::default(),
            rgb_enabled: true,
            last_party: Instant::now(),
        }
    }

    async fn on_layer_change_event(&mut self, _event: LayerChangeEvent) {
        // GRIN RC's touch controls are independent of the active keyboard layer.
    }

    async fn poll(&mut self) {
        let now = embassy_time::Instant::now();
        let now_ms = now.as_millis() as u32;
        let left = [
            self.left[0].is_low(),
            self.left[1].is_low(),
            self.left[2].is_low(),
            self.left[3].is_low(),
        ];
        let right = [
            self.right[0].is_low(),
            self.right[1].is_low(),
            self.right[2].is_low(),
            self.right[3].is_low(),
        ];
        let center = self.center.is_low();

        if let Some(event) = self.left_state.update(left, now_ms, true) {
            self.handle_touch(event);
        }
        if let Some(event) = self.right_state.update(right, now_ms, false) {
            self.handle_touch(event);
        }
        if let Some(event) = self.center_state.update(center, now_ms) {
            self.handle_touch(event);
        }

        // Keep the QMK party mode concept: a continuously changing hue, but only
        // transmit a frame every 40 ms so the LED refresh cannot starve RMK.
        if self.mode == SliderMode::Party && now.duration_since(self.last_party) >= Duration::from_millis(40) {
            self.last_party = now;
            self.hsv.h = (self.hsv.h + 3) % 360;
            self.render_leds();
        }
    }

    fn handle_touch(&mut self, event: TouchEvent) {
        match event {
            TouchEvent::CenterTap => {
                self.mode = match self.mode {
                    SliderMode::None => SliderMode::Rgb,
                    SliderMode::Rgb => SliderMode::Volume,
                    SliderMode::Volume => SliderMode::Scroll,
                    SliderMode::Scroll => SliderMode::Party,
                    SliderMode::Party => SliderMode::None,
                };
                if self.mode == SliderMode::Party {
                    self.last_party = Instant::now();
                }
                self.render_leds();
            }
            TouchEvent::CenterLongPress => {
                self.rgb_enabled = !self.rgb_enabled;
                self.render_leds();
            }
            TouchEvent::LeftTap => {
                if self.mode == SliderMode::Rgb || self.mode == SliderMode::Party {
                    self.hsv.h = (self.hsv.h + 330) % 360;
                    self.render_leds();
                }
            }
            TouchEvent::RightTap => {
                if self.mode == SliderMode::Rgb || self.mode == SliderMode::Party {
                    self.hsv.h = (self.hsv.h + 30) % 360;
                    self.render_leds();
                }
            }
            TouchEvent::LeftSlide { position, delta, .. } => {
                match self.mode {
                    SliderMode::Rgb | SliderMode::Party => {
                        self.hsv.v = rgb_value_for_slider(position);
                        self.render_leds();
                    }
                    SliderMode::Scroll => self.publish_scroll(delta, 0),
                    SliderMode::Volume => {
                        // Volume HID actions are deliberately left to the next
                        // integration step; no physical matrix position is
                        // borrowed for a virtual key.
                    }
                    SliderMode::None => {}
                }
            }
            TouchEvent::RightSlide { position, delta, .. } => {
                match self.mode {
                    SliderMode::Rgb | SliderMode::Party => {
                        self.hsv.h = hue_for_slider(position);
                        self.render_leds();
                    }
                    SliderMode::Scroll => self.publish_scroll(0, delta),
                    SliderMode::Volume => {}
                    SliderMode::None => {}
                }
            }
        }
    }

    fn publish_scroll(&self, dx: i16, dy: i16) {
        if dx == 0 && dy == 0 {
            return;
        }
        publish_event(PointingEvent {
            device_id: ALL_POINTING_DEVICES,
            axes: [
                AxisEvent { typ: AxisValType::Rel, axis: Axis::H, value: dx },
                AxisEvent { typ: AxisValType::Rel, axis: Axis::V, value: dy },
                AxisEvent { typ: AxisValType::Rel, axis: Axis::Z, value: 0 },
            ],
        });
    }

    fn render_leds(&mut self) {
        // Temporarily disabled for USB/touch isolation.
        // The GRIN touch processor must remain responsive without entering the
        // long WS2812 bit-banging critical section.
    }

    #[inline(always)]
    fn write_byte(&mut self, value: u8) {
        for bit in (0..8).rev() {
            let one = (value & (1 << bit)) != 0;
            self.led.set_high();
            if one {
                cortex_m::asm::delay(WS2812_T1H_CYCLES);
                self.led.set_low();
                cortex_m::asm::delay(WS2812_BIT_CYCLES - WS2812_T1H_CYCLES);
            } else {
                cortex_m::asm::delay(WS2812_T0H_CYCLES);
                self.led.set_low();
                cortex_m::asm::delay(WS2812_BIT_CYCLES - WS2812_T0H_CYCLES);
            }
        }
    }
}

fn hsv_to_rgb(h: u16, s: u8, v: u8) -> (u8, u8, u8) {
    if s == 0 {
        return (v, v, v);
    }
    let region = (h / 60) % 6;
    let f = ((h % 60) * 255 / 60) as u8;
    let p = ((v as u16 * (255 - s as u16)) / 255) as u8;
    let q = ((v as u16 * (255 - (s as u16 * f as u16 / 255))) / 255) as u8;
    let t = ((v as u16 * (255 - (s as u16 * (255 - f as u16) / 255))) / 255) as u8;
    match region {
        0 => (v, t, p),
        1 => (q, v, p),
        2 => (p, v, t),
        3 => (p, q, v),
        4 => (t, p, v),
        _ => (v, p, q),
    }
}
