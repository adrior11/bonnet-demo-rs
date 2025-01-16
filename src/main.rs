mod position;

use embedded_graphics::{
    geometry::Point,
    pixelcolor::BinaryColor,
    prelude::*,
    primitives::{Circle, PrimitiveStyle, Rectangle},
};
use linux_embedded_hal::I2cdev;
use rppal::gpio::Gpio;
use ssd1306::{
    mode::BufferedGraphicsMode, prelude::*, size::DisplaySize, I2CDisplayInterface, Ssd1306,
};

use position::*;

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
    // Side buttons
    let button_pin_a = gpio.get(5).unwrap().into_input_pullup();
    let button_pin_b = gpio.get(6).unwrap().into_input_pullup();
    // Directional stick
    let button_pin_l = gpio.get(27).unwrap().into_input_pullup();
    let button_pin_r = gpio.get(23).unwrap().into_input_pullup();
    let button_pin_u = gpio.get(17).unwrap().into_input_pullup();
    let button_pin_d = gpio.get(22).unwrap().into_input_pullup();

    let mut pos = Position::new(64, 32);

    // TODO: Improve button processing like in raycast-rs
    loop {
        if button_pin_l.is_low() {
            pos.try_move(-1, 0);
            draw_circle(&mut display, &pos);
        } else if button_pin_r.is_low() {
            pos.try_move(1, 0);
            draw_circle(&mut display, &pos);
        } else if button_pin_u.is_low() {
            pos.try_move(0, -1);
            draw_circle(&mut display, &pos);
        } else if button_pin_d.is_low() {
            pos.try_move(0, 1);
            draw_circle(&mut display, &pos);
        }

        if button_pin_a.is_low() {
            draw_circle(&mut display, &pos);
        } else if button_pin_b.is_low() {
            clear_screen(&mut display);
        }
    }

    // TODO: clear on CTRL-C
}

fn draw_circle<DI, SIZE>(
    display: &mut Ssd1306<DI, SIZE, BufferedGraphicsMode<SIZE>>,
    pos: &Position,
) where
    DI: WriteOnlyDataCommand,
    SIZE: DisplaySize,
{
    let style_fg = PrimitiveStyle::with_fill(BinaryColor::On);
    let circle = Circle::new(pos.to_point(), DIAMETER).into_styled(style_fg);
    circle.draw(display).unwrap();
    display.flush().unwrap();
}

fn clear_screen<DI, SIZE>(display: &mut Ssd1306<DI, SIZE, BufferedGraphicsMode<SIZE>>)
where
    DI: WriteOnlyDataCommand,
    SIZE: DisplaySize,
{
    let style = PrimitiveStyle::with_fill(BinaryColor::Off);
    let clear_area = Rectangle::new(Point::new(0, 0), Size::new(127, 63));
    clear_area.into_styled(style).draw(display).unwrap();
    display.flush().unwrap();
}
