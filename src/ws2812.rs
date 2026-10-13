//! WS2812 waveform encoder, independent of the MCU transmit backend.
//!
//! Each output sample is a PWM compare value for one WS2812 bit period.
//! The hardware backend must configure a timer period of approximately 1.25 us,
//! DMA these samples to the timer compare register, and hold the data line low
//! for the reset/latch interval after the last pixel.
//!
//! WS2812 pixels are transmitted in GRB byte order, most-significant bit first.
//! This module allocates no memory and does not disable interrupts.

/// One pixel in normal RGB channel order.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Rgb8 {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Rgb8 {
    pub const fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }
}

/// Errors returned by the allocation-free waveform encoder.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EncodeError {
    /// Output buffer is too short for the encoded pixels and reset samples.
    BufferTooSmall { required: usize, available: usize },
    /// The high pulse for a zero bit must be shorter than for a one bit.
    InvalidPulseWidths,
}

/// Number of PWM samples required for a frame, including low reset samples.
pub const fn required_samples(pixel_count: usize, reset_samples: usize) -> usize {
    pixel_count * 24 + reset_samples
}

/// Encode a pixel frame into PWM compare values.
///
/// zero_high and one_high are timer ticks for the high pulse of a zero or
/// one bit. The timer period itself is configured by the hardware backend.
/// reset_samples zero-duty samples keep the line low for the WS2812 latch
/// interval; choose this from the configured timer frequency (at least 50 us
/// for classic WS2812B parts; check the exact LED revision's datasheet).
///
/// The function validates its inputs before modifying out. The returned
/// length is the number of samples written; any trailing elements are untouched.
pub fn encode_frame(
    pixels: &[Rgb8],
    out: &mut [u16],
    zero_high: u16,
    one_high: u16,
    reset_samples: usize,
) -> Result<usize, EncodeError> {
    if zero_high >= one_high {
        return Err(EncodeError::InvalidPulseWidths);
    }

    let required = required_samples(pixels.len(), reset_samples);
    if out.len() < required {
        return Err(EncodeError::BufferTooSmall {
            required,
            available: out.len(),
        });
    }

    let mut cursor = 0;
    for pixel in pixels {
        // WS2812 wire order is Green, Red, Blue (not RGB).
        for byte in [pixel.g, pixel.r, pixel.b] {
            for bit in (0..8).rev() {
                out[cursor] = if byte & (1 << bit) != 0 {
                    one_high
                } else {
                    zero_high
                };
                cursor += 1;
            }
        }
    }

    // Zero compare values hold the output low for reset/latch.
    out[cursor..required].fill(0);
    Ok(required)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn emits_grb_msb_first_then_reset_low() {
        let pixels = [Rgb8::new(0x12, 0x80, 0x01)];
        let mut out = [999u16; 27];
        assert_eq!(encode_frame(&pixels, &mut out, 3, 7, 3), Ok(27));
        assert_eq!(&out[..8], &[7, 3, 3, 3, 3, 3, 3, 3]); // G = 0x80
        assert_eq!(&out[8..16], &[3, 3, 3, 7, 3, 3, 7, 3]); // R = 0x12
        assert_eq!(&out[16..24], &[3, 3, 3, 3, 3, 3, 3, 7]); // B = 0x01
        assert_eq!(&out[24..], &[0, 0, 0]);
    }

    #[test]
    fn rejects_short_buffer_without_mutating_it() {
        let pixels = [Rgb8::new(1, 2, 3)];
        let mut out = [55u16; 26];
        assert_eq!(
            encode_frame(&pixels, &mut out, 3, 7, 3),
            Err(EncodeError::BufferTooSmall { required: 27, available: 26 })
        );
        assert!(out.iter().all(|&sample| sample == 55));
    }

    #[test]
    fn rejects_invalid_pulse_widths() {
        let mut out = [0u16; 24];
        assert_eq!(
            encode_frame(&[Rgb8::default()], &mut out, 7, 7, 0),
            Err(EncodeError::InvalidPulseWidths)
        );
    }
}
