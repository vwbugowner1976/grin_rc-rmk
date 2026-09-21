# grin_rc-rmk

RMK firmware port for GRIN RC.

Upstream hardware/behavior reference:
- https://github.com/policium/grin_rc/tree/main/firmware/source/grin_keebs/grin_rc

Development happens on `rmk-port-v0.1`.
The goal is feature parity with the QMK firmware, including:
- STM32F411 keyboard matrix
- Rynk / MyKeebStudio integration
- US-on-JIS key behavior
- WS2812 RGB matrix
- SSD1306 OLED
- left/right/center capacitive touch controls
