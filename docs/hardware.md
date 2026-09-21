# GRIN RC hardware reference

Source of truth for the initial port is the upstream QMK tree:

- https://github.com/policium/grin_rc/tree/main/firmware/source/grin_keebs/grin_rc

## MCU

- STM32F411
- QMK board target: `BLACKPILL_STM32_F411`
- QMK bootloader: `stm32-dfu`
- USB VID: `0x476B`
- USB PID: `0x0002`

## Matrix

- 10 rows x 7 columns
- diode direction: COL2ROW
- rows: PB13, PB10, PB14, PB12, PB15, PB0, PB2, PA4, PB1, PA3
- columns: PA14, PA15, PB3, PB4, PB5, PB6, PB7

## I2C

QMK uses I2C1:

- SCL: PB8
- SDA: PB9
- external EEPROM: 24LC64
- OLED: SSD1306-family display

The QMK OLED renderer writes 512-byte frames, consistent with 128 x 32 monochrome output.

## RGB

- WS2812-compatible chain
- data: PA0
- LED count: 66
- QMK maximum brightness: 200

## Capacitive touch

The QMK firmware treats the touch electrodes as active-low digital inputs with pull-ups.

Left slider:
- PA13
- PA10
- PA9
- PA8

Right slider:
- PC14
- PC13
- PC15
- PA2

Center:
- PA1

Slider position weights in the QMK implementation are 100, 75, 50 and 25.

## QMK touch behavior to preserve

- center tap: cycle operating mode
- center long press: RGB toggle
- left/right tap hooks
- RGB mode
- volume mode
- horizontal/vertical scroll mode
- party/RGB animation mode
