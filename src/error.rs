use core::fmt::Formatter;

// use embedded_hal::i2c::{I2c, SevenBitAddress};
use embedded_hal::i2c::{Error as I2cError, ErrorKind as I2cErrorKind};

/// All possible errors
/// Display not implemented for no_std support
#[derive(Clone, Copy, Debug)]
pub enum Error<E> {
    NotConnected,
    /// parameter out of range
    OutOfRange(u8),
    /// measurement timeout
    MeasurementTimeout(),
    /// An error in the  underlying I²C system
    I2c(E),
}

// impl<E> From<E> for Error<E>
// where
//     E: I2cError,
// {
//     fn from(error: E) -> Self {
//         Self::I2c(error.kind())
//     }
// }


/****
pub enum Si7021Error<I2C>
where
    I2C: I2c<SevenBitAddress>
{
    /// Error during I2C write operation.
    WriteError(I2C::Error),
    /// Error during I2C WriteRead operation.
    WriteReadError(I2C::Error),
    /// Got an unexpected Part Id during sensor initalization.
    UnexpectedChipId(u8),

    /// sths34pf80 device not connected to I2C bus
    NotConnected,
    /// parameter out of range
    OutOfRange(u8),
    /// measurement timeout
    MeasurementTimeout(),
}

impl<I2C> core::fmt::Debug for Si7021Error<I2C>
where
    I2C: I2c<SevenBitAddress>
{
    fn fmt(&self, f: &mut Formatter<'_>) -> core::result::Result<(), core::fmt::Error> {
        match self {
            Si7021Error::WriteReadError(e) => f.debug_tuple("WriteReadError").field(e).finish(),
            Si7021Error::WriteError(e) => f.debug_tuple("WriteError").field(e).finish(),
            Si7021Error::UnexpectedChipId(chip_id) => f
                .debug_tuple("Expected part id 0xd3, got : ") // ToDo:  fix this one
                .field(chip_id)
                .finish(),
            Si7021Error::NotConnected => f
                .debug_tuple("Si7021 series device is not connected to microcontroller")
                .finish(),

            Si7021Error::OutOfRange(value) => f
                .debug_tuple("Set value out of range, check Si7021 datasheet")
                .field(value)
                .finish(),
            Si7021Error::MeasurementTimeout() => f
                .debug_tuple("timeout waiting for new measurement data ready")
                .finish(),
        }
    }
}


*************/