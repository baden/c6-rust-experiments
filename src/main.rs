use esp_idf_sys as _; // If using the `binstart` feature of `esp-idf-sys`, always keep this module imported
use log::*;

use esp_idf_hal::delay::FreeRtos;
use esp_idf_hal::i2c::*;
use esp_idf_hal::peripherals::Peripherals;
use esp_idf_hal::prelude::*;

use embedded_graphics::{
    mono_font::{ascii::FONT_10X20, MonoTextStyleBuilder},
    pixelcolor::BinaryColor,
    prelude::*,
    text::{Baseline, Text},
    primitives::{Rectangle, PrimitiveStyleBuilder}
};
// use embedded_graphics::pixelcolor::*;
use ssd1306::{
    // mode::BufferedGraphicsMode,
    prelude::*,
    I2CDisplayInterface,
    Ssd1306
};

// const SSD1306_ADDRESS: u8 = 0x3c;

fn main() {
    let mut counter: u32 = 0;
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_sys::link_patches();
    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    let per = Peripherals::take().unwrap();
    let i2c = per.i2c0;
    let pins = per.pins;
    let sda = pins.gpio5;
    let scl = pins.gpio6;

    println!("Starting I2C SSD1306 example...");

    let config = I2cConfig::new().baudrate(100.kHz().into());
    let i2c = I2cDriver::new(i2c, sda, scl, &config).unwrap();

    let interface = I2CDisplayInterface::new(i2c);
    let mut display = Ssd1306::new(
        interface,
        DisplaySize128x32,
        DisplayRotation::Rotate0
    )
    .into_buffered_graphics_mode();
    display.init().unwrap();
    display.clear(BinaryColor::Off).unwrap();
    display.flush().unwrap();

    let text_style = MonoTextStyleBuilder::new()
        .font(&FONT_10X20)
        .text_color(BinaryColor::On)
        .build();

    Text::with_baseline(
        "Hello, world!",
        Point::zero(),
        text_style,
        Baseline::Top
    ).draw(&mut display).unwrap();

    display.flush().unwrap();

    info!("Hello, world!");

    let clear = PrimitiveStyleBuilder::new()
        .stroke_color(BinaryColor::Off)
        .fill_color(BinaryColor::Off)
        .build();

    loop {
        Rectangle::new(
            Point::new(0, 16),
            Size::new(128, 16)
        ).into_styled(clear).draw(&mut display).unwrap();
        Text::with_baseline(
            &format!("Counter: {}", counter),
            Point::new(0, 16),
            text_style,
            Baseline::Top
        ).draw(&mut display).unwrap();
        counter += 1;

        display.flush().unwrap();
        FreeRtos::delay_ms(1000);
    }    


}
