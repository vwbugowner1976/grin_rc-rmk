# QMK → RMK parity checklist

Reference firmware: [policium/grin_rc](https://github.com/policium/grin_rc), branch `main`.

Primary source files:
- `firmware/source/grin_keebs/grin_rc/keymaps/default/keymap.c`
- `firmware/source/grin_keebs/grin_rc/rgb_matrix_kb.inc`
- `firmware/source/grin_keebs/grin_rc/config.h`

## Confirmed QMK behavior

- 66 WS2812 LEDs on PA0; QMK's custom effect is `dj_effect`.
- Touch inputs are active-low.
- Left and right slider electrodes use a 400-count threshold; center uses 300.
- Slider taps are detected within 300 ms; sliding begins after 500 ms.
- Center tap cycles NONE → RGB → VOL → SCROLL → PARTY → NONE.
- Center long press (800 ms) toggles the RGB matrix.
- RGB mode: left slider changes brightness; right slider changes hue; left/right taps step the RGB effect backward/forward.
- Volume mode: right slider emits volume up/down.
- Scroll mode: left slider emits horizontal wheel; right slider emits vertical wheel.
- PARTY mode: left slider changes brightness; right slider changes `rgb_angle` by `delta * 3`.
- `rgb_matrix_kb.inc` defines `dj_effect`, which calculates hue from each LED's physical `dx/dy` angle plus `rgb_angle`.

## RMK port status (source inspection only; not a hardware verification)

- [x] Pure WS2812 GRB/MSB-first PWM waveform encoder and unit tests (`src/ws2812.rs`).
- [x] Pure touch and RGB slider mapping modules with unit tests (`src/touch.rs`, `src/rgb.rs`).
- [ ] Main currently registers `SingleTouchTest`, not the full `GrinControl`.
- [ ] `GrinControl::render_leds()` is empty; no timer/DMA backend is wired to PA0.
- [ ] Volume-mode HID actions are not implemented.
- [ ] QMK `dj_effect` / physical LED coordinates / `rgb_angle` behavior are not ported.
- [ ] RMK OLED behavior has not been verified against the QMK status display.
- [ ] Full firmware build, USB enumeration, and physical touch/LED tests must be recorded separately.

## Next steps (keep changes small)

1. Implement the STM32F411 timer/DMA transport for PA0 without taking over the Embassy time-driver timer.
2. Unit-test frame generation for all 66 LEDs and the reset-low interval.
3. Wire `GrinControl` into `main.rs` only after the transport is compile-checked.
4. Add volume HID behavior and port the `dj_effect` hue math using QMK's LED positions.
5. Build/flash and verify USB, matrix, OLED, touch gestures, and LEDs on hardware.
6. Only then mark hardware acceptance gates complete or open the upstream RMK PR.
