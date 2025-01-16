# Adafruit OLED Bonnet Demo

A minimal **Rust** application for testing out the **Adafruit 1.3” OLED Bonnet (SSD1306)**
with a **Raspberry Pi (5)**. 

## 🎨 Overview

This project demonstrates:
- Using [linux-embedded-hal](https://github.com/rust-embedded/linux-embedded-hal) and [rppal](https://github.com/golemparts/rppal) for `I2C` and `GPIO` access.
- Drawing on the OLED display with [embedded-graphics](https://github.com/embedded-graphics/embedded-graphics) and [ssd1306](https://github.com/rust-embedded-community/ssd1306).
- Handling button/joystick inputs.
- Exiting `SIGINT` cleanly via [ctrlc](https://github.com/Detegr/rust-ctrlc).

## 🔌 Hardware Setup
### Adafruit 1.3" OLED Bonnet

- Plug the bonnet onto your Raspberry Pi's GPIO header.
- Confirm that the bonnet's SDA/SCL pins map to the PI's `I2C1`.
- Typically, this is on pins 3 (SDA) and 5 (SCL).

### Enable I2C
- Run `sudo raspi-config`, go to `Interface > Options > I2C` enable it, then reboot.
- Confirm the device with:
```bash
$ sudo i2cdetect -y 1

# This should output something like:
#      0  1  2  3  4  5  6  7  8  9  a  b  c  d  e  f
# 00:                         -- -- -- -- -- -- -- -- 
# 10: -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- 
# 20: -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- 
# 30: -- -- -- -- -- -- -- -- -- -- -- -- 3c -- -- -- 
# 40: -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- 
# 50: -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- 
# 60: -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- 
# 70: -- -- -- -- -- -- -- --    

```
> [!IMPORTANT]
> The demo uses `0x3c` per default. You might need to adjust the code, if this differs.

### Buttons and Stick

- This bonnet has 7 GPIO-based inputs (2 buttons, 4-direction joystick, and pressing the joystick).
- In the demo, we assume these pins:
```
Button A        -> GPIO 5
Button B        -> GPIO 6
Joystick Left   -> GPIO 27
Joystick Right  -> GPIO 23
Joystick Up     -> GPIO 17
Joystick Down   -> GPIO 22
Joystick Press  -> GPIO 4   # Not used in code
```
> [!NOTE]
> Check Adafruit's documentation or pinout if your hardware differs for your setup.

## ⚙️ Software Requirements
- **Rust**
- **Cargo**
- **I2C Tools** (i2c-tools, libi2c-dev on Debian-based distros)

Make sure you can build Rust projects on your Pi or [cross-compile](https://github.com/cross-rs/cross) from another machine.
You can find the installation documentation for Rust and Cargo [here](https://www.rust-lang.org/tools/install).

## 🚀 Running the Demo
1. Clone this repository:
```bash
$ git clone https://github.com/adrior11/bonnet-demo-rs.git
```
2. Build and Run:
```bash
$ cargo run
```

## 🕹️ Usage
### Controlls
- **Joystick:** Move the circle around (left, right, up, down).
- **Button A:** Redraw the circle if needed.
- **Button B:** Clear the display.
### Exiting
- Press `Ctrl+C` in the terminal to trigger a cleanup routine that clears the display and shuts down the application.

## 📁 Project Structure
```bash
.
├── ...
├── Cargo.toml      # Project metadata & dependencies
└── src
    ├── main.rs     # Main logic: sets up display, pins, main loop
    └── position.rs # Position structure with movement logic
```

## License
This project is available under the MIT License.
