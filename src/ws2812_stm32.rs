//! STM32F411 WS2812 output using timer PWM + DMA.
//!
//! GRIN RC's QMK configuration uses the standard WS2812 driver on PA0 with
//! 66 LEDs and a brightness limit of 200. This RMK backend uses Embassy's
//! timer-update DMA waveform API so LED transmission does not bit-bang with
//! interrupts disabled.
//!
//! TIM5_CH1 is available on PA0 (AF2) on STM32F411. TIM3 is reserved for
//! embassy-time by Cargo features, so the LED PWM timer cannot steal its clock.

use embassy_stm32::{
    bind_interrupts, dma,
    gpio::OutputType,
    peripherals,
    time::khz,
    timer::{
        Channel,
        low_level::CountingMode,
        simple_pwm::{PwmPin, SimplePwm},
    },
    Peri,
};
use rmk::macros::processor;

use crate::ws2812::{encode_frame, EncodeError, Rgb8};

const LED_COUNT: usize = 66;
const RESET_SAMPLES: usize = 40; // 50 us at 800 kHz
const SAMPLE_COUNT: usize = LED_COUNT * 24 + RESET_SAMPLES;

bind_interrupts!(struct Ws2812Irqs {
    DMA1_STREAM0 => dma::InterruptHandler<peripherals::DMA1_CH0>;
});

/// PWM+DMA driver for the GRIN RC's WS2812 chain on PA0.
pub struct Ws2812Dma<'d> {
    pwm: SimplePwm<'d, peripherals::TIM5>,
    dma: Peri<'d, peripherals::DMA1_CH0>,
    samples: [u16; SAMPLE_COUNT],
    zero_high: u16,
    one_high: u16,
}

impl<'d> Ws2812Dma<'d> {
    /// Initialize TIM5_CH1 on PA0 at the WS2812 800 kHz bit rate.
    pub fn new(
        tim5: Peri<'d, peripherals::TIM5>,
        pa0: Peri<'d, peripherals::PA0>,
        dma: Peri<'d, peripherals::DMA1_CH0>,
    ) -> Self {
        let pin = PwmPin::new(pa0, OutputType::PushPull);
        let mut pwm = SimplePwm::new(
            tim5,
            Some(pin),
            None,
            None,
            None,
            khz(800),
            CountingMode::EdgeAlignedUp,
        );
        pwm.channel(Channel::Ch1).set_duty_cycle(0);

        // With a 1.25 us bit period, 38/120 ~= 0.40 us and 76/120 ~= 0.79 us.
        // These are within the commonly used WS2812B timing windows.
        let period = pwm.max_duty_cycle() as u16;
        let zero_high = (period as u32 * 32 / 100) as u16;
        let one_high = (period as u32 * 64 / 100) as u16;

        Self {
            pwm,
            dma,
            samples: [0; SAMPLE_COUNT],
            zero_high,
            one_high,
        }
    }

    /// Send a pixel slice in GRB wire order, followed by a 50 us low reset.
    ///
    /// This function does not allocate and does not disable interrupts for the
    /// entire frame. The caller should pass no more than 66 pixels.
    pub async fn write_pixels(&mut self, pixels: &[Rgb8]) -> Result<(), EncodeError> {
        let len = encode_frame(
            pixels,
            &mut self.samples,
            self.zero_high,
            self.one_high,
            RESET_SAMPLES,
        )?;

        self.pwm
            .waveform_up(
                self.dma.reborrow(),
                Ws2812Irqs,
                Channel::Ch1,
                &self.samples[..len],
            )
            .await;

        Ok(())
    }
}

/// Temporary hardware bring-up processor: cycles low-brightness RGB colors.
///
/// Kept separate from touch handling so PA0/timer/DMA can be verified first.
#[processor(poll_interval = 500)]
pub struct Ws2812SmokeTest<'d> {
    driver: Ws2812Dma<'d>,
    phase: u8,
}

impl<'d> Ws2812SmokeTest<'d> {
    pub fn new(
        tim5: Peri<'d, peripherals::TIM5>,
        pa0: Peri<'d, peripherals::PA0>,
        dma: Peri<'d, peripherals::DMA1_CH0>,
    ) -> Self {
        Self {
            driver: Ws2812Dma::new(tim5, pa0, dma),
            phase: 0,
        }
    }

    async fn poll(&mut self) {
        // Intentionally dim colors for safe first power-on testing.
        let color = match self.phase % 4 {
            0 => Rgb8::new(16, 0, 0),
            1 => Rgb8::new(0, 16, 0),
            2 => Rgb8::new(0, 0, 16),
            _ => Rgb8::new(0, 0, 0),
        };
        let pixels = [color; LED_COUNT];
        if let Err(_error) = self.driver.write_pixels(&pixels).await {
            // A fixed-size buffer sized for 66 pixels should never fail.
            // Avoid panicking in the input processor if that invariant changes.
        }
        self.phase = self.phase.wrapping_add(1);
    }
}
