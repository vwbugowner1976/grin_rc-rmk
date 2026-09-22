# GRIN RC hardware

Source of truth: policium/grin_rc QMK firmware.

MCU: STM32F411.

Matrix: COL2ROW, 10 rows / 7 columns.
Rows: PB13, PB10, PB14, PB12, PB15, PB0, PB2, PA4, PB1, PA3.
Columns: PA14, PA15, PB3, PB4, PB5, PB6, PB7.

RGB: WS2812 data PA0, 66 LEDs.
I2C1: SCL PB8, SDA PB9. EEPROM: 24LC64.

Left touch: PA13, PA10, PA9, PA8.
Right touch: PC14, PC13, PC15, PA2.
Center touch: PA1.

OLED is used by the QMK firmware and reports WPM, layer and touch mode.
