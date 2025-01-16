use embedded_graphics::{
    pixelcolor::BinaryColor,
    prelude::*,
    primitives::{Circle, PrimitiveStyle, Rectangle},
};
use linux_embedded_hal::I2cdev;
use rppal::gpio::Gpio;
use ssd1306::{
    mode::BufferedGraphicsMode, prelude::*, size::DisplaySize, I2CDisplayInterface, Ssd1306,
};

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

    let gpio = Gpio::new().unwrap();
    let button_pin_a = gpio.get(5).unwrap().into_input_pullup();
    let button_pin_b = gpio.get(6).unwrap().into_input_pullup();

    loop {
        if button_pin_a.is_low() {
            toggle_circle(&mut display, true);
        } else if button_pin_b.is_low() {
            toggle_circle(&mut display, false);
        }
    }
}

fn toggle_circle<DI, SIZE>(
    display: &mut Ssd1306<DI, SIZE, BufferedGraphicsMode<SIZE>>,
    on: bool,
) where
    DI: WriteOnlyDataCommand,
    SIZE: DisplaySize,
{
    let bin_color = if on {
        BinaryColor::On
    } else {
        BinaryColor::Off
    };
    let style = PrimitiveStyle::with_fill(bin_color);

    if on {
        let circle = Circle::new(Point::new(64, 32), 20).into_styled(style);
        circle.draw(display).unwrap();
    } else {
        let clear_area = Rectangle::new(Point::new(0, 0), Size::new(127, 63));
        clear_area.into_styled(style).draw(display).unwrap();
    }

    display.flush().unwrap();
}
