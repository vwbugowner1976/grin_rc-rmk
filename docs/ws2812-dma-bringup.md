# WS2812 DMA bring-up (STM32F411 / RMK 0.9)

## Reference and design choice

The GRIN RC QMK config defines the WS2812 data pin as PA0 and 66 LEDs, with a maximum brightness of 200. The upstream rules.mk selects RGB_MATRIX_DRIVER = WS2812; its optional PWM-driver settings in config.h are commented out. The RMK implementation therefore reuses the protocol requirements rather than copying a QMK C driver into Rust.

Embassy STM32 0.6 provides a timer-PWM + DMA waveform API. This branch uses:
- PA0 / TIM5_CH1 (alternate function 2) for the serial data waveform
- TIM5 update DMA via DMA1 stream 0 / DMA1_CH0
- 800 kHz bit periods
- GRB byte order, MSB first
- 66 pixels and 40 low-duty reset samples (50 us at 800 kHz)
- TIM3 reserved explicitly for embassy-time, avoiding timer ownership overlap

The existing pure src/ws2812.rs encoder remains MCU-independent. src/ws2812_stm32.rs is the hardware backend and a temporary smoke-test processor.

## Current status

- [x] Locate upstream pin/count/brightness settings.
- [x] Add an allocation-free PWM sample encoder.
- [x] Add an Embassy timer-update DMA backend and low-brightness RGB smoke pattern.
- [x] Reserve TIM3 for the Embassy time driver so TIM5 remains available to the LED backend.
- [ ] Compile the branch with cargo check --target thumbv7em-none-eabihf.
- [ ] Confirm the DMA request mapping against the generated STM32F411 peripheral map and successful firmware build.
- [ ] Flash the board and confirm red, green, blue, then off on all 66 LEDs.
- [ ] Replace the temporary smoke-test processor with calls from the GRIN touch/RGB control processor.
- [ ] Verify keyboard USB, matrix, OLED and touch behavior alongside LED refresh.

GitHub Actions currently reports no workflow runs for this branch, so there is no CI pass claimed. Do not treat the backend as hardware-verified until the build and physical checks above pass.

## First hardware test

The current branch registers a temporary processor that cycles all LEDs through dim red, dim green, dim blue, then off at roughly 500 ms intervals. The colors use channel value 16 to avoid an unexpectedly bright first power-on. If the firmware build succeeds, flash this branch and inspect the LEDs; the touch processor remains separate during this isolation test.

If the build fails, resolve compile/API and DMA-map errors before connecting the renderer to touch events. If it builds but no LEDs light, check PA0 signal wiring, LED power/ground, DIN direction, and the first LED's reset/latch requirements before changing timing constants.
