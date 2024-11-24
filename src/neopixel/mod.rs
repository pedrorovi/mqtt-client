use std::borrow::BorrowMut;

use esp_idf_hal::gpio::*;
use esp_idf_hal::peripheral::*;
use esp_idf_hal::peripherals::Peripherals;
use esp_idf_hal::rmt::RmtChannel;
use smart_leds::{SmartLedsWrite, White, RGB8};
use ws2812_esp32_rmt_driver::*;

pub struct NeoPixel {
    driver: LedPixelEsp32Rmt<'static, RGBW8, driver::color::LedPixelColorGrbw32>,
}

impl NeoPixel {
    pub fn new<C: RmtChannel, P: OutputPin>(
        channel: impl Peripheral<P = C> + 'static,
        pin: impl Peripheral<P = P> + 'static,
    ) -> Self {
        let driver =
            LedPixelEsp32Rmt::new(channel, pin).expect("Failed to initialize WS2812 driver");
        NeoPixel { driver }
    }

    pub fn set_color(&mut self, color: RGBW8) {
        self.driver
            .write_nocopy([color].iter().cloned())
            .expect("Failed to write to WS2812");
    }
}

pub fn get_color(red: u8, green: u8, blue: u8, white: u8) -> RGBW8 {
    RGBW8 {
        r: red,
        g: green,
        b: blue,
        a: White(white),
    }
}

pub fn get_logging_rgbw8() -> RGBW8 {
    RGBW8 {
        r: 0,
        g: 50,
        b: 0,
        a: White(0),
    }
}

pub fn get_error_rgbw8() -> RGBW8 {
    RGBW8 {
        r: 50,
        g: 0,
        b: 0,
        a: White(0),
    }
}

pub fn get_sleeping_rgbw8() -> RGBW8 {
    RGBW8 {
        r: 0,
        g: 0,
        b: 50,
        a: White(0),
    }
}