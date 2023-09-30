use esp_idf_sys as _; // If using the `binstart` feature of `esp-idf-sys`, always keep this module imported
use log::*;

// use esp_idf_hal::delay::{FreeRtos, BLOCK};
use esp_idf_hal::{i2c::*};
use esp_idf_hal::peripherals::Peripherals;
use esp_idf_hal::prelude::*;

use embedded_graphics::{
    mono_font::{ascii::FONT_10X20, MonoTextStyleBuilder},
    pixelcolor::BinaryColor,
    prelude::*,
    text::{Baseline, Text},
};
use ssd1306::{
    // mode::BufferedGraphicsMode,
    prelude::*,
    I2CDisplayInterface,
    Ssd1306
};

// const SSD1306_ADDRESS: u8 = 0x3c;

fn main() {
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
    // display.clear();
    display.flush().unwrap();

    let text_style = MonoTextStyleBuilder::new()
        .font(&FONT_10X20)
        .text_color(BinaryColor::On)
        .build();

    Text::with_baseline(
        "Hello world!",
        Point::zero(),
        text_style,
        Baseline::Top
    ).draw(&mut display).unwrap();

    display.flush().unwrap();

    info!("Hello, world!");

    // initialze the display - don't worry about the meaning of these bytes - it's specific to SSD1306
    // i2c.write(SSD1306_ADDRESS, &[0, 0xae], BLOCK).unwrap();
    // i2c.write(SSD1306_ADDRESS, &[0, 0xd4], BLOCK).unwrap();
    // i2c.write(SSD1306_ADDRESS, &[0, 0x80], BLOCK).unwrap();
    // i2c.write(SSD1306_ADDRESS, &[0, 0xa8], BLOCK).unwrap();
    // i2c.write(SSD1306_ADDRESS, &[0, 0x3f], BLOCK).unwrap();
    // i2c.write(SSD1306_ADDRESS, &[0, 0xd3], BLOCK).unwrap();
    // i2c.write(SSD1306_ADDRESS, &[0, 0x00], BLOCK).unwrap();
    // i2c.write(SSD1306_ADDRESS, &[0, 0x40], BLOCK).unwrap();
    // i2c.write(SSD1306_ADDRESS, &[0, 0x8d], BLOCK).unwrap();
    // i2c.write(SSD1306_ADDRESS, &[0, 0x14], BLOCK).unwrap();
    // i2c.write(SSD1306_ADDRESS, &[0, 0xa1], BLOCK).unwrap();
    // i2c.write(SSD1306_ADDRESS, &[0, 0xc8], BLOCK).unwrap();
    // i2c.write(SSD1306_ADDRESS, &[0, 0xda], BLOCK).unwrap();
    // i2c.write(SSD1306_ADDRESS, &[0, 0x12], BLOCK).unwrap();
    // i2c.write(SSD1306_ADDRESS, &[0, 0x81], BLOCK).unwrap();
    // i2c.write(SSD1306_ADDRESS, &[0, 0xcf], BLOCK).unwrap();
    // i2c.write(SSD1306_ADDRESS, &[0, 0xf1], BLOCK).unwrap();
    // i2c.write(SSD1306_ADDRESS, &[0, 0xdb], BLOCK).unwrap();
    // i2c.write(SSD1306_ADDRESS, &[0, 0x40], BLOCK).unwrap();
    // i2c.write(SSD1306_ADDRESS, &[0, 0xa4], BLOCK).unwrap();
    // i2c.write(SSD1306_ADDRESS, &[0, 0xa6], BLOCK).unwrap();
    // i2c.write(SSD1306_ADDRESS, &[0, 0xaf], BLOCK).unwrap();
    // i2c.write(SSD1306_ADDRESS, &[0, 0x20, 0x00], BLOCK).unwrap();

    // // fill the display
    // for _ in 0..64 {
    //     let data: [u8; 17] = [
    //         0x40, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
    //         0xff, 0xff, 0xff,
    //     ];
    //     i2c.write(SSD1306_ADDRESS, &data, BLOCK).unwrap();
    // }

    // loop {
    //     // we are sleeping here to make sure the watchdog isn't triggered
    //     FreeRtos::delay_ms(500);
    //     i2c.write(SSD1306_ADDRESS, &[0, 0xa6], BLOCK).unwrap();
    //     FreeRtos::delay_ms(500);
    //     i2c.write(SSD1306_ADDRESS, &[0, 0xa7], BLOCK).unwrap();
    // }    
}
