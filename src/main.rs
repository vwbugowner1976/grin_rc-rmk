#![no_main]
#![no_std]

use defmt_rtt as _;
use panic_probe as _;

mod rgb;
mod touch;
mod us_on_jis;

use rmk::macros::rmk_keyboard;

#[rmk_keyboard]
mod keyboard {
    use embassy_stm32::Config;
    use embassy_stm32::time::Hertz;

    #[Override(chip_config)]
    fn config() -> Config {
        // GRIN RC uses the STM32F411 BlackPill target in the upstream QMK tree.
        // RMK's STM32F4 example uses this RCC setup for STM32F411 + 25 MHz HSE.
        let mut config = Config::default();
        {
            use embassy_stm32::rcc::*;

            config.rcc.hse = Some(Hse {
                freq: Hertz(25_000_000),
                mode: HseMode::Oscillator,
            });
            config.rcc.pll_src = PllSource::HSE;
            config.rcc.pll = Some(Pll {
                prediv: PllPreDiv::DIV25,
                mul: PllMul::MUL192,
                divp: Some(PllPDiv::DIV2),
                divq: Some(PllQDiv::DIV4),
                divr: None,
            });
            config.rcc.ahb_pre = AHBPrescaler::DIV1;
            config.rcc.apb1_pre = APBPrescaler::DIV2;
            config.rcc.apb2_pre = APBPrescaler::DIV1;
            config.rcc.sys = Sysclk::PLL1_P;
        }
        config
    }
}
