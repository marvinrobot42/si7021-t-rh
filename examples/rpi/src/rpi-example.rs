use anyhow::Result;
use linux_embedded_hal::{Delay, I2cdev};

use si7021_t_rh::Si7021;

use std::thread;
use std::time::Duration;

use env_logger::Builder;
use log::{LevelFilter, error, info};
use std::io::Write;

fn main() -> Result<()> {

    let mut builder = Builder::from_default_env();

    builder
        .format(|buf, record| writeln!(buf, "{} - {}", record.level(), record.args()))
        .filter(None, LevelFilter::Info)
        .init();


    info!("Hello, linux Rust and Si70xx series world!");

    let dev_i2c = I2cdev::new("/dev/i2c-1").unwrap();
    let delayer = Delay {};

    let mut my_si7021 = Si7021::new(i2c_bus, delayer);
    my_si7021.init_device().unwrap();
    
    info!("Si70xx read device ID = {:#?} and firmware version = {}, serial_a = {:#08x}, serial_b = {:#08x}",
    my_si7021.device_data.id, my_si7021.device_data.firmware_version, 
    my_si7021.device_data.serial_a, my_si7021.device_data.serial_b);

    let humidity: f32 = my_si7021.read_relative_humidity().unwrap();
    info!("humidity is {}", humidity);


    let temperature: f32 = my_si7021.read_temperature().unwrap();
    info!("temperature is {}", temperature);
    FreeRtos::delay_ms(50);



    loop {
        info!("sleep looping");
        thread::sleep(Duration::from_secs(10));
        info!("Si70xx measurements are {:#?}", my_si7021.read_measurements().unwrap());
    
    }

}
