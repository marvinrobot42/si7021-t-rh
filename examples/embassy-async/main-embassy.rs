#![no_std]
#![no_main]

/****
 *  Cargo.toml dependencies:  
 * 
 *  for ESP32-C6 no_std and embassy async
 * 
 * [dependencies]
esp-backtrace = { version = "0.13.0", features = [
    "esp32c6",
    "exception-handler",
    "panic-handler",
    "println",
] }
esp-hal = { version = "0.19.0", features = [ "esp32c6", "async", "log"] }
esp-hal-embassy = { version = "0.2.0", features = [ "esp32c6" , "log" , "integrated-timers"] }
embedded-svc = { version = "0.26.1", default-features = false, features = [] }
embassy-executor   = { version = "0.5.0", features = [ "integrated-timers", "arch-riscv32"] }
embassy-sync        = "0.5.0"
embassy-time        = "0.3.0"
esp-println = { version = "0.10.0",  default-features = false, features = ["esp32c6", "jtag-serial", "log"] }
esp-alloc = { version = "0.3.0" }
log = { version = "0.4.21" }
heapless = { version = "0.8.0", default-features = false }
static_cell = "2.1.0"
embedded-hal-async = "1.0.0"

si7021-t-rh = { version = "0.1", features = ["async"] }
 * 
 * 
 */


use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};
use esp_backtrace as _;
use esp_hal::{
    clock::ClockControl,
    gpio::{Io, Level, Output},
    i2c::I2C,
    peripherals::Peripherals,
    prelude::*,
    system::SystemControl,
    timer::{timg::TimerGroup, ErasedTimer, OneShotTimer},
};

use si7021_t_rh::Si7021;

use log::{info, debug};

// When you are okay with using a nightly compiler it's better to use https://docs.rs/static_cell/2.1.0/static_cell/macro.make_static.html
macro_rules! mk_static {
    ($t:ty,$val:expr) => {{
        static STATIC_CELL: static_cell::StaticCell<$t> = static_cell::StaticCell::new();
        #[deny(unused_attributes)]
        let x = STATIC_CELL.uninit().write(($val));
        x
    }};
}

#[main]
async fn main(_spawner: Spawner) {
    let peripherals = Peripherals::take();
    let system = SystemControl::new(peripherals.SYSTEM);
    let clocks = ClockControl::boot_defaults(system.clock_control).freeze();

    let timg0 = TimerGroup::new(peripherals.TIMG0, &clocks, None);
    let timer0 = OneShotTimer::new(timg0.timer0.into());
    let timers = [timer0];
    let timers = mk_static!([OneShotTimer<ErasedTimer>; 1], timers);
    esp_hal_embassy::init(&clocks, timers);

    let io = Io::new(peripherals.GPIO, peripherals.IO_MUX);

    let i2c0 = I2C::new_async(
        peripherals.I2C0,
        io.pins.gpio6,   // ESP32-c6 mini
        io.pins.gpio7,   // ESP32-c6 mini
        400.kHz(),
        &clocks,
    );

    //esp_println::logger::init_logger_from_env();
    esp_println::logger::init_logger(log::LevelFilter::Debug);
    info!("info logging!");
    debug!("debug logging");

    // Si7021 
    let mut my_si7021 = Si7021::new(i2c0, embassy_time::Delay);
    my_si7021.init_device().await.unwrap();

    info!("Si70xx read device ID = {:#?} and firmware version = {}, serial_a = {:#08x}, serial_b = {:#08x}",
    my_si7021.device_data.id, my_si7021.device_data.firmware_version, 
    my_si7021.device_data.serial_a, my_si7021.device_data.serial_b);

    let humidity: f32 = my_si7021.read_relative_humidity().await.unwrap();
    info!("humidity is {}", humidity);

    Timer::after(Duration::from_millis(100)).await;

    let temperature: f32 = my_si7021.read_temperature().await.unwrap();
    info!("temperature is {}", temperature);
    Timer::after(Duration::from_millis(100)).await;



    let mut led_pin23 = Output::new(io.pins.gpio23, Level::High);
    led_pin23.toggle();
    Timer::after(Duration::from_millis(2000)).await;
    led_pin23.toggle();


    loop {

        let measurements = my_si7021.read_measurements().await.unwrap();
        info!("Si7021 measurements are {:#?}", measurements);

        led_pin23.toggle();
        Timer::after(Duration::from_millis(2000)).await;
    }

}