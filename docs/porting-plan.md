# RMK Phase 4 parity plan

This repository intentionally does not treat a keyboard-only port as completion.
The target is feature parity with the existing QMK firmware plus
Rynk/MyKeebStudio integration.

## Required for v0.1 hardware validation

- [ ] STM32F411 boots RMK over USB
- [ ] 10x7 matrix matches the QMK physical layout
- [ ] Rynk enumerates and MyKeebStudio connects
- [ ] keymap read/write works
- [ ] consumer and mouse HID reports work
- [ ] US-on-JIS punctuation translation works
- [ ] SSD1306 128x32 works on I2C1 PB8/PB9
- [ ] 66-LED WS2812 chain works on PA0
- [ ] left four-electrode slider works
- [ ] right four-electrode slider works
- [ ] center touch tap/hold works
- [ ] RGB / volume / scroll / party modes match upstream behavior
- [ ] touch settings can be exposed to MyKeebStudio

## Architecture

The RMK keyboard task remains responsible for normal matrix/HID/Rynk behavior.
GRIN-specific peripherals should run as independent Embassy tasks and publish
actions into the keyboard/HID path.

```text
matrix ----------------> RMK keyboard/Rynk -----> USB HID
                            ^
                            |
touch scan -> gestures -> mode/action router
                |           |      |      |
                |           |      |      +-> RGB task
                |           |      +--------> consumer HID
                |           +---------------> mouse wheel H/V
                +---------------------------> OLED/status
```

## Porting rule

Do not silently replace QMK behavior with a merely similar feature. When exact
behavior is not yet ported, keep it explicitly marked incomplete and retain the
upstream QMK implementation as the behavioral reference.
