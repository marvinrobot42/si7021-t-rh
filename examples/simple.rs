// ESP32-C6 based example

/**** Cargo.toml includes:

[dependencies]
log = { version = "0.4", default-features = false }
embedded-svc = "0.25.0"
esp-idf-svc = { version = "0.48", default-features = false }
esp-idf-sys = { version = "0.34.0", features = ["binstart"] }
esp-idf-hal = "0.43.0"

anyhow = "1.0.75"
# no good si7021 = "0.2.0"
si7021-t-rh = {"0.1" }

[build-dependencies]
embuild = "0.31.3"

****************/

use anyhow::Result;

use esp_idf_hal::{
    delay::{Ets, FreeRtos}, i2c::{I2cConfig, I2cDriver}, io::Error, prelude::*
};
use esp_idf_hal::{gpio::PinDriver, prelude::Peripherals};
use esp_idf_hal::peripheral;

use esp_idf_hal::delay::Delay;


use esp_idf_sys::{self as _};
use log::{info, error};

use si7021_t_rh::Si7021;


fn main() -> Result<()> {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    log::info!("Hello, world!");

    let peripherals = Peripherals::take().unwrap();
    let pins = peripherals.pins;
    let sda = pins.gpio6; // esp32-c3  has pins.gpio0 , check your board schematic
    let scl = pins.gpio7; // esp32-c3  haspins.gpio1, check your board schematic
    let i2c = peripherals.i2c0;
    let config = I2cConfig::new().baudrate(400.kHz().into());
    let i2c_bus = I2cDriver::new(i2c, sda, scl, &config)?;
    
    let mut led_pin23 = PinDriver::output(pins.gpio23).unwrap();  // check your board schematic for LED pin #
    led_pin23.set_high().unwrap();
    
    let mut delay: Delay = Default::default();
    info!("calling S17021:: new");
    let mut my_si7021 = Si7021::new(i2c_bus, delay);
    info!("calling si7021 reset()");
    //my_si7021.reset_device().unwrap();  ... init will call reset_device()
    my_si7021.init_device().unwrap();

    info!("Si70xx read device ID = {:#?} and firmware version = {}, serial_a = {:#08x}, serial_b = {:#08x}",
        my_si7021.device_data.id, my_si7021.device_data.firmware_version, 
        my_si7021.device_data.serial_a, my_si7021.device_data.serial_b);

    let humidity: f32 = my_si7021.read_relative_humidity().unwrap();
    info!("humidity is {}", humidity);


    let temperature: f32 = my_si7021.read_temperature().unwrap();
    info!("temperature is {}", temperature);
    FreeRtos::delay_ms(50);

    my_si7021.heater_control(true);
    info!("is heater enabled = {:#?}", my_si7021.is_heater_enabled());

    my_si7021.set_heater_level(0x04).unwrap();
    
    FreeRtos::delay_ms(100);  // turn on heater to 100 msec
    my_si7021.heater_control(false);
    info!("is heater enabled = {:#?}", my_si7021.is_heater_enabled());
    FreeRtos::delay_ms(50);  // need a delay before reading humidity again ...in loop below

    loop {
        // the read_measurements() method is a little faster (-30 msec) than reading humidity and temperature separately
        info!("Si70xx measurements are {:#?}", my_si7021.read_measurements().unwrap());
        FreeRtos::delay_ms(10000);
    }

    Ok(())

}
