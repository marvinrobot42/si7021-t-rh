// no_std support
#[allow(unused_imports)]
#[allow(dead_code)]
// use libm::{exp, round, trunc};
use log::debug;

#[allow(unused_imports)] // for no_std use
//use num_traits::float::FloatCore;

//use crate::error::Ens160Error;
// use bitfield::bitfield;


/// A measurement result from the sensor.
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Measurements {
    /// temperature degrees C
    pub temperature_c: f32,
    /// relative humidity percent
    pub relative_humidity_percent: f32,
}

/// Si7021 device id and firmware version
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct DeviceData {
    /// device ID
    pub id: DeviceModel, 
    pub firmware_version: u8,
    pub serial_a: u32,
    pub serial_b: u32,
}

impl Default for DeviceData {
    fn default() -> Self {
        DeviceData {
            id: DeviceModel::NotRead,
            firmware_version: 0,
            serial_a: 0,
            serial_b: 0,
        }
    }
}

/// SiLabs device model
#[derive(Debug, PartialEq, Clone, Copy)]
#[repr(u8)]
pub enum DeviceModel {
    SiEngineeringSample = 0xff,
    Si7013 = 0x0d,
    Si7020 = 0x14,
    Si7021 = 0x15,
    NotRead = 0x00,
}

impl From<u8> for DeviceModel {
    fn from(v: u8) -> Self {
        match v {
            0xff => Self::SiEngineeringSample,
            0x0d => Self::Si7013,
            0x14 => Self::Si7020,
            0x15 => Self::Si7021,
            0x00 => Self::NotRead,
            _ => unreachable!(),
        }
    }
}





