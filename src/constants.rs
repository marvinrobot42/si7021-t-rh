// Si7021 registers

#![allow(nonstandard_style)]
pub const Si7021_READ_RH_HOLD: u8 = 0xE5;  // Measure Relative Humidity, Hold Master Mode 
pub const Si7021_READ_RH_NO_HOLD: u8 = 0xF5;  // Measure Relative Humidity, No Hold Master Mode
pub const Si7021_READ_TEMP_HOLD: u8 = 0xE3;  // Measure Temperature, Hold Master Mode
pub const Si7021_READ_TEMP_NO_HOLD: u8 = 0xF3; // Measure Temperature, No Hold Master Mode
pub const Si7021_READ_TEMP_AFTER_PREVIOUS_RH: u8 = 0xE0; // Read Temperature Value from Previous RH Measurement
pub const Si7021_RESET: u8 = 0xFE;  // Reset
pub const Si7021_WRITE_RH_T_USER_REG_1  : u8 = 0xE6;  // Write RH/T User Register 1
pub const Si7021_READ_RH_T_USER_REG_1  : u8 = 0xE7;  // Read RH/T User Register 1
pub const Si7021_WRITE_HEATER_CONTROL: u8 = 0x51;  // Write Heater Control Register
pub const Si7021_READ_HEATER_CONTROL: u8 = 0x11;  // READ Heater Control Register
pub const Si7021_READ_ID_BYTE_1: [u8; 2] = [0xFA, 0x0F];  // Read Electronic ID 1st Byte
pub const Si7021_READ_ID_BYTE_2: [u8; 2] = [0xFC, 0xC9];  // Read Electronic ID 2nd Byte
pub const Si7021_READ_FW_VERSION: [u8; 2] = [0x84, 0xB8];  // Read Firmware Revision



#[repr(u8)]
/// Si7021 I2C device address
#[derive(Debug, Clone, Copy)]
pub enum DeviceAddress {
    /// it only has one I2C address but let's call it primary in case future secondary is needed
    Primary = 0x40,  
}

impl From<DeviceAddress> for u8 {
    fn from(value: DeviceAddress) -> Self {
        match value {
            DeviceAddress::Primary => 0x40,
        }
    }
}

impl Default for DeviceAddress {
    fn default() -> Self {
        Self::Primary
    }
}
