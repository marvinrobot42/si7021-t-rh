# Si7021-T-RH &emsp; 
[![crates.io](https://img.shields.io/crates/v/si7021-t-rh)](https://crates.io/crates/si7021-t-rh)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](https://github.com/marvinrobot42/si7021-t-rh)
[![Documentation](https://docs.rs/si7021-t-rh/badge.svg)](https://docs.rs/si7021-t-rh)

## A Rust crate for Silicon Labs Si70xx series sensor (7013, 7020, 7021)
<https://github.com/marvinrobot42/si7021-t-rh.git>

[Si7021]: https://www.silabs.com/sensors/humidity/si7006-13-20-21-34/device.si7021-a20-gm?tab=specs

The Si7013, 7020, 7021 sensor relative humidity and temperature sensor with I2C interface.

All I2C API functions are implemented. 

### Features

- uses embedded-hal version 1.0.x
- async support included (see examples folder in repository for embassy-async ESP32-C6 example)
- designed for embedded use (ESP32-C3, -C6 and -S3 and Raspberry Pi)
- ESP32 RISC V and Raspberry Pi examples included
- enable/disable builtin heater for clearing condensation or frost (when not reading temperature)
- configurable heater power level
- reads device model number, serial numbers and firmware version
- an easy to use Measurements struct
- no_std embedded compatible

  

#### Notes

Developed using Adafruit Si7021 STEMMA QT (QWIIC) model: https://www.adafruit.com/product/3251


### Recent version history
  - 0.1.4  Added async support, cleaned out compiler warnings
  - 0.1.3  Updated README.md
  - 0.1.2  Fixed repo name (whoops it is s17021-t-rh instead of si7021)
  - 0.1.1  Fixed README.md
  - 0.1.0  Initial release


## Usage
----

Add the dependency to `Cargo.toml`.

~~~~toml
[dependencies.si7021-t-rh]
version = "0.1"
~~~~

1. Create a hardward specific I²C driver interface and delay function
2. Create an Si7021 struct with the I²C interface and a delay function as parameters.  
3. Initialize the Si7021 instance
4.a Read relative humidity
4.b Read temperature or
5. Call read_measurements() fn which return both relative humdity and temperature faster than
  call the seperate methods due to Si70xx "Read Temperature Value from Previous RH Measurement function" 
 


### Simple Example

A more complete example is in the repository examples path
~~~~rust

use anyhow::Result;  // add dependency for this
use si7021_t_rh::Si7021;
use log::{info, error};

...


fn main() -> Result<()> {

  ...as per your hardware hal, below is ESP32-C3 - C6 style

  let peripherals = Peripherals::take().unwrap();
  let pins = peripherals.pins;
  let sda = pins.gpio6; // esp32-c3  has pins.gpio0 , check your board schematic
  let scl = pins.gpio7; // esp32-c3  haspins.gpio1, check your board schematic
  let i2c = peripherals.i2c0;
  let config = I2cConfig::new().baudrate(400.kHz().into());
  let i2c_bus = I2cDriver::new(i2c, sda, scl, &config)?;

  let mut delay: Delay = Default::default();   // your hardware delay from use ...
  let mut my_si7021 = Si7021::new(i2c_bus, delay);
  my_si7021.init_device().unwrap();
  FreeRtos::delay_ms(250);  // or your sleep function here

  loop {
    // the read_measurements() method is a little faster (-30 msec) than reading humidity and temperature separately
    log::info!("Si70xx measurements are {:#?}", my_si7021.read_measurements().unwrap());
    FreeRtos::delay_ms(10000);
  }

}
    
~~~~

### For async set si7021-t-rh dependency features = ["async"] and Si7021::new method requires async I2C and delay 
###    parameters.  Default features is sync (blocking)


### License
----

You are free to copy, modify, and distribute this application with attribution under the terms of either

 * Apache License, Version 2.0
   ([LICENSE-Apache-2.0](./LICENSE-Apache-2.0) or <https://opensource.org/licenses/Apache-2.0>)
 * MIT license
   ([LICENSE-MIT](./LICENSE-MIT) or <https://opensource.org/licenses/MIT>)

at your option.

This project is not affiliated with nor endorsed in any way by Silicon Labs or Adafruit.
