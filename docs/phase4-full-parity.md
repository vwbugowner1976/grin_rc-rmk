# GRIN RC RMK Phase 4 — full-parity target

The target is a behavioral port of the upstream QMK GRIN RC firmware, with RMK/Rynk as the configuration layer.

## Hardware contract

- MCU: STM32F411
- Matrix: 10 rows × 7 columns, COL2ROW
- USB VID/PID: 0x476B / 0x0002
- WS2812: 66 LEDs, PA0
- OLED: SSD1306 128×32, I2C1 PB8/PB9, address 0x3C
- Touch: left PA13/PA10/PA9/PA8, right PC14/PC13/PC15/PA2, center PA1
- EEPROM: 24LC64

## QMK behavior preserved

- 3 user layers: base, function/navigation, system
- four-electrode weighted positions: 100/75/50/25
- slider debounce counter: 400
- center debounce counter: 300
- slider tap <= 300 ms
- slider slide begins after 500 ms
- center long press at 800 ms
- center tap cycles NONE → RGB → VOL → SCROLL → PARTY → NONE
- center long press toggles RGB
- RGB mode: left controls brightness, right controls hue
- RGB mode: left/right tap changes RGB mode
- VOL mode: right slider changes volume
- SCROLL mode: left = horizontal wheel, right = vertical wheel
- PARTY mode: left controls brightness and right changes animation angle
- QMK starts RGB in solid color mode
- OLED displays WPM, active layer and touch mode

The pure touch/RGB modules now encode the QMK timing and slider bucket rules
and include host unit tests. Hardware GPIO/WS2812/HID plumbing is kept isolated
until the exact RMK 0.9 embedded APIs are compile-tested.

## US-on-JIS

USB HID usages are physical usage identifiers; a Japanese host layout may
interpret those usages differently. The port must not claim that simply sending
LeftBracket guarantees '[' on every Japanese OS.

src/us_on_jis.rs contains the US punctuation usage table. The final transport
implementation needs an explicit host policy for Windows/macOS/Linux.

## MyKeebStudio

Rynk is the intended native configuration path. It supports keymap/layer
editing, combos, tap-dance/morse, macros, status, reboot and bootloader/reset
operations. Vial and Rynk are mutually exclusive, so this branch selects Rynk.

## Acceptance gates

1. cargo check --target thumbv7em-none-eabihf
2. STM32F411 USB enumeration
3. 10×7 matrix and 66-key physical layout
4. Rynk connection and persistent keymap write
5. consumer + mouse HID
6. SSD1306 output
7. 66-LED WS2812 output
8. all three touch surfaces
9. exact slider timing/threshold behavior
10. RGB / volume / horizontal+vertical scroll / party behavior
11. MyKeebStudio read/write integration
12. DFU recovery path

No item is complete merely because a similar feature exists in RMK.
