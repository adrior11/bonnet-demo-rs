use embedded_graphics::{
    pixelcolor::BinaryColor,
    prelude::*,
    primitives::{Circle, PrimitiveStyle, Rectangle},
};
use linux_embedded_hal::I2cdev;
use ssd1306::{prelude::*, I2CDisplayInterface, Ssd1306};

fn main() {
    // Set up I2C device on Raspberry Pi 5 
    match I2cdev::new("/dev/i2c-1") {
        Ok(_) => println!("Successfully opened /dev/i2c-1"),
        Err(e) => eprintln!("Failed to open /dev/i2c-1: {:?}", e),
    }

    let i2c = I2cdev::new("/dev/i2c-1").expect("Failed to open i2c bus");

    let interface = I2CDisplayInterface::new(i2c);
    let mut display = Ssd1306::new(interface, DisplaySize128x64, DisplayRotation::Rotate0)
        .into_buffered_graphics_mode();
    display.init().unwrap();

    let style = PrimitiveStyle::with_fill(BinaryColor::On);
    let circle = Circle::new(Point::new(64, 32), 20).into_styled(style);
    circle.draw(&mut display).unwrap();
    display.flush().unwrap();

    let style = PrimitiveStyle::with_fill(BinaryColor::Off);
    let clear_area = Rectangle::new(Point::new(0, 0), Size::new(127, 63));
    clear_area.into_styled(style).draw(&mut display).unwrap();
    display.flush().unwrap();
}
