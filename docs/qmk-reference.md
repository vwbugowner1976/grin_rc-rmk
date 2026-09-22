# QMK migration contract

The RMK port is intended to preserve the original QMK behavior.

Touch modes: NONE, RGB, VOL, SCROLL, PARTY.

Center touch: tap cycles mode; long press toggles RGB.

Left slider: RGB brightness, horizontal scroll, party brightness; tap changes RGB mode in RGB mode.

Right slider: RGB hue, volume, vertical scroll, party RGB-angle behavior; tap changes RGB mode in RGB mode.

Timing reference: left/right tap window 300 ms, slide around 500 ms, center long press 800 ms. Original electrode debounce counters use thresholds 400 (left/right) and 300 (center).

Each side has four electrodes mapped to 25/50/75/100 positions and multiple active electrodes are averaged.

The firmware must also preserve US-layout behavior while Windows remains configured as Japanese. The existing JTU custom-keycode implementation is the reference for that behavior.
