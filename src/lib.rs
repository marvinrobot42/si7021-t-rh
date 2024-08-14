#![no_std]
#![allow(dead_code)]
#![allow(unused_variables)]

//#![feature(inherent_associated_types)]
pub mod error;

use crate::error::Error;

pub mod data;
use crate::data::DeviceData;

pub mod constants;

use crate::constants::DeviceAddress::Primary;

use constants::{Si7021_READ_FW_VERSION, Si7021_READ_ID_BYTE_1, Si7021_READ_ID_BYTE_2, Si7021_READ_RH_NO_HOLD, Si7021_READ_RH_T_USER_REG_1, Si7021_READ_TEMP_AFTER_PREVIOUS_RH, Si7021_READ_TEMP_NO_HOLD, Si7021_RESET, Si7021_WRITE_HEATER_CONTROL, Si7021_WRITE_RH_T_USER_REG_1};
use data::{DeviceModel, Measurements};

#[cfg(not(feature = "async"))]
use embedded_hal::{i2c::I2c, delay::DelayNs};
#[cfg(feature = "async")]
use embedded_hal_async::{i2c::I2c as AsyncI2c, delay::DelayNs as AsyncDelayNs};

use log::{debug, info};




//#[allow(unused_imports)]
//use embedded_hal::{delay::DelayNs, i2c::I2c, i2c::ErrorType};
//// use libm::{powf, truncf};
//use log::{debug, info};


/// the Si7021 device
pub struct Si7021<I2C, D> {
    /// I²C interface
    i2c: I2C,
    /// I²C device address
    address: u8,
    delayer: D,
    pub device_data: DeviceData,
}

#[cfg(not(feature = "async"))]
impl<I2C, D, E> Si7021<I2C, D>
where  
    I2C: I2c<Error = E>,
    D: DelayNs,
{
    
    //type Error = Error;
    /// create new Si7021 driver with default I2C address: ADDR pin low
    pub fn new(i2c: I2C, delayer: D) -> Self {
        log::debug!("new called");
        Self {
            i2c,
            address: Primary.into(),
            delayer,
            device_data: DeviceData { ..Default::default()},
        }
    }

    /// give back the I2C interface
    pub fn release(self) -> I2C {
        self.i2c
    }

}

#[cfg(feature = "async")]
impl<I2C, D, E> Si7021<I2C, D>
where  
    I2C: AsyncI2c<Error = E>,
    D: AsyncDelayNs,
{
    //type Error = Error;
    /// create new Si7021 driver with default I2C address: ADDR pin low
    pub fn new(i2c: I2C, delayer: D) -> Self {
        debug!("new called");
        Self {
            i2c,
            address: Primary.into(),
            delayer,
            device_data: DeviceData { ..Default::default()},
        }
    }

    /// give back the I2C interface
    pub fn release(self) -> I2C {
        self.i2c
    }

}

#[maybe_async_cfg::maybe(
    sync(
        cfg(not(feature = "async")),
        self = "Si7021",
        idents(AsyncI2c(sync = "I2c"), AsyncDelayNs(sync = "DelayNs"))
    ),
    async(feature = "async", keep_self)
)]

impl<I2C, D, E> Si7021<I2C, D>
where  
    I2C: AsyncI2c<Error = E>,
    D: AsyncDelayNs,
{


    // command_buf is an u8 array that starts with command byte followed by command data byte(s)
    async fn write_command<const N: usize>(&mut self, command_buf: [u8; N] ) -> Result<(), Error<E>> {
        // debug!("write_command : {:#?}", command_buf);
        self.i2c
            .write(self.address, &command_buf).await
            .map_err(Error::I2c)?;
        Ok(())
    }

    async fn read_register( &mut self, register_address: u8, buffer: &mut [u8] ) -> Result<(), Error<E>> {
        let mut command_buffer = [0u8; 1];
        command_buffer[0] = register_address;
        // let mut result_buffer = [0u8; N];
        self.i2c
            .write_read(self.address, &command_buffer, buffer).await
            .map_err(Error::I2c)?;
        Ok(())
    }


    /// check if Si7021 is connected by reading Read RH/T User Register 1
    pub async fn is_connected(&mut self) -> Result<bool, Error<E>> {
        debug!("in is_connected()");
        let mut result_buf: [u8; 1] = [0; 1];
        self.read_register(Si7021_READ_RH_T_USER_REG_1, &mut result_buf).await?;
        debug!(" is_connected() {:#04x}, expected 0x3a", result_buf[0]);
        #[allow(unused_parens)]
        if (result_buf[0] == 0x3a) {  // why 0x3a ?
            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// reset S17021 device, called by init_device()
    pub async fn reset_device(&mut self) -> Result<(), Error<E>> {
        debug!("in reset_device()");
        self.write_command([Si7021_RESET]).await?;
        debug!("wrote reset command");
        self.delayer.delay_ms(50).await;
        Ok(())   
    }

    /// initial S17021 device
    pub async fn init_device(&mut self) -> Result<bool, Error<E>> {
        debug!("in init_device()");
        self.reset_device().await?;
        #[allow(unused_parens)]
        if (!self.is_connected().await?) {
            return Ok(false);
        } else {
            debug!("device is connected");
        }
        #[allow(unused_parens)]
        if (self.read_device_model().await? == DeviceModel::NotRead) {
            debug!("device model is not read");
            return Ok(false);
        }
        let firmware_result = self.read_firmware_version().await?;
        debug!("fw version is {:#?}", firmware_result);
        Ok(true)
    }

    /// read firmware version number of Si7021
    pub async fn read_firmware_version(&mut self) -> Result<u8, Error<E>> {
        let mut result_buf: [u8; 1] = [0; 1];
        let command_buffer : [u8; 2] = [Si7021_READ_FW_VERSION[0], Si7021_READ_FW_VERSION[1]];
        self.i2c
            .write_read(self.address, &command_buffer, &mut result_buf).await
            .map_err(Error::I2c)?;
        #[allow(unused_parens)]
        if (result_buf[0] == 0xff) {
            self.device_data.firmware_version = 1;
        } else if (result_buf[0] == 0x20) {
            self.device_data.firmware_version = 2;
        } else {
            self.device_data.firmware_version = 0;  // is this an invalid value?
        }
        Ok(self.device_data.firmware_version.clone())
    }

    /// read device model 
    pub async fn read_device_model(&mut self) -> Result<DeviceModel, Error<E>> {
        let mut result_buf: [u8; 8] = [0; 8];
        let mut command_buffer: [u8; 2] = [Si7021_READ_ID_BYTE_1[0], Si7021_READ_ID_BYTE_1[1]];
        self.i2c
            .write_read(self.address, &command_buffer, &mut result_buf).await
            .map_err(Error::I2c)?;
        let serial_id_a: u32 = u32::from_be_bytes([result_buf[0], result_buf[2], result_buf[4], result_buf[6] ]);
        self.device_data.serial_a = serial_id_a;
        debug!("serial_id_a = {}", serial_id_a);

        command_buffer = [Si7021_READ_ID_BYTE_2[0], Si7021_READ_ID_BYTE_2[1]];
        self.i2c
            .write_read(self.address, &command_buffer, &mut result_buf).await
            .map_err(Error::I2c)?;

        let serial_id_b: u32 = u32::from_be_bytes( [result_buf[0], result_buf[1], result_buf[4], result_buf[5] ]);
        self.device_data.serial_b = serial_id_b;
        debug!("serial_id_b = {}", serial_id_b);
        info!("device model byte is {:#04x} = {:#?}", result_buf[0], DeviceModel::from(result_buf[0]));
        
        self.device_data.id = DeviceModel::from(result_buf[0]);  // second user register byte 0 
        Ok(DeviceModel::from(result_buf[0]))
    }

    /// read relative humidity in percent
    pub async fn read_relative_humidity(&mut self) -> Result<f32, Error<E>> {
        debug!("in read_relative_humidity()");
        self.delayer.delay_ms(20).await;  // in case user called this again too quickly
        let command_buffer: [u8; 1] = [Si7021_READ_RH_NO_HOLD]; 
        self.i2c.write(self.address, &command_buffer).await
            .map_err(Error::I2c)?;
        self.delayer.delay_ms(25).await;
        debug!("did i2c write, next is read");
        let mut result_buf: [u8; 3] = [0; 3];
        self.i2c.read(self.address, &mut result_buf).await
            .map_err(Error::I2c)?;
        
        let humidity_u16 : u16 = u16::from_be_bytes( [result_buf[0], result_buf[1] ]);
        // scale it
        let mut humidity: f32 = ( humidity_u16 as f32 * 125.0 / 65536.0 ) - 6.0;
        
        // clamp value to => 0 and <= 100
        #[allow(unused_parens)]
        if (humidity.is_sign_negative() ) {
            humidity = 0.0;
        }
        #[allow(unused_parens)]
        if (humidity > 100.0) {
            humidity = 100.0
        }

        Ok(humidity)
        

    }


    /// read temperatue in degrees C
    pub async fn read_temperature(&mut self) -> Result<f32, Error<E>> {
        debug!("in read_temperature()");
        let command_buffer: [u8; 1] = [Si7021_READ_TEMP_NO_HOLD]; 
        self.i2c.write(self.address, &command_buffer).await
                .map_err(Error::I2c)?;
        self.delayer.delay_ms(20).await;
        debug!("did i2c write, next is read");
        let mut result_buf: [u8; 3] = [0; 3];
        self.i2c.read(self.address, &mut result_buf).await
                .map_err(Error::I2c)?;
        
        let temperature_u16 : u16 = u16::from_be_bytes( [result_buf[0], result_buf[1] ]);
        // scale it
        let temperature: f32 = ( temperature_u16 as f32 * 175.72 / 65536.0 ) - 46.85;

        Ok(temperature)
    }

    /// read measurements (temperature and humidity as a struct)
    pub async fn read_measurements(&mut self) -> Result<Measurements, Error<E>> {
        debug!("in read_measurements()");
        let humidity = self.read_relative_humidity().await?;
        //self.delayer.delay_ms(5);
        let mut result_buf: [u8; 2] = [0; 2];
        self.read_register(Si7021_READ_TEMP_AFTER_PREVIOUS_RH, &mut result_buf).await?;
        let temperature_u16 : u16 = u16::from_be_bytes( [result_buf[0], result_buf[1] ]);
        // scale it
        let temperature: f32 = ( temperature_u16 as f32 * 175.72 / 65536.0 ) - 46.85;

        let measurements: Measurements = Measurements {
            relative_humidity_percent: humidity,
            temperature_c: temperature,
        };
        Ok(measurements)
    }  
  
    /// heater control enable/disable
    pub async fn heater_control(&mut self, enable: bool) -> Result<(), Error<E>> {
        debug!("in heater_control({})", enable);
        let mut result_buf: [u8; 1] = [0; 1];
        self.read_register(Si7021_READ_RH_T_USER_REG_1, &mut result_buf).await?;
        let mut write_value = result_buf[0];
        if enable {
            write_value = write_value | (1 << 2);
        } else {
            write_value = write_value & !(1 << 2);
        }
        self.write_command([Si7021_WRITE_RH_T_USER_REG_1, write_value]).await?;

        Ok(())
    }

    /// is heater enabled
    pub async fn is_heater_enabled(&mut self) -> Result<bool, Error<E>> {
        debug!("in is_heater_enabled()");
        let mut result_buf: [u8; 1] = [0; 1];
        self.read_register(Si7021_READ_RH_T_USER_REG_1, &mut result_buf).await?;
        let read_value = result_buf[0];
        #[allow(unused_parens)]
        if ((read_value & 0x04) != 0x00)  {
            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// set heater power level
    pub async fn set_heater_level(&mut self, level: u8) -> Result<(), Error<E>> {
        debug!("in set_heater_level({})", level);
        #[allow(unused_parens)]
        if (level > 0x0f) {
            return Err(Error::OutOfRange(level));
        }
        self.write_command([Si7021_WRITE_HEATER_CONTROL, level]).await?;
        Ok(())
    }

}