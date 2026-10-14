#![no_std]

// Pure modules are exposed as a library so their unit tests can run on the host
// without compiling the STM32 firmware binary.
pub mod rgb;
pub mod touch;
pub mod ws2812;
