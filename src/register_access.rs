use crate::{Error, Rv3029};
use embedded_hal::i2c::blocking::I2c;

pub struct Register;
impl Register {
    pub const SECONDS: u8 = 0x08;
    pub const MINUTES: u8 = 0x09;
    pub const HOURS: u8 = 0x0A;
    pub const DOM: u8 = 0x0B;
    pub const DOW: u8 = 0x0C;
    pub const MONTH: u8 = 0x0D;
    pub const YEAR: u8 = 0x0E;
    // pub const SQWOUT: u8 = 0x07;
    pub const EEPROM_BEGIN: u8 = 0x28;
    pub const EEPROM_END: u8 = 0x29;
    pub const RAM_BEGIN: u8 = 0x38;
    pub const RAM_END: u8 = 0x3F;
}

pub struct BitFlags;
impl BitFlags {
    pub const H24_H12: u8 = 0b0100_0000;
    pub const AM_PM: u8 = 0b0010_0000;
    // pub const SQWE: u8 = 0b0001_0000;
    // pub const OUTLEVEL: u8 = 0b1000_0000;
    // pub const OUTRATERS0: u8 = 0b0000_0001;
    // pub const OUTRATERS1: u8 = 0b0000_0010;
}

pub const ADDR: u8 = 0xac;

impl<I2C> Rv3029<I2C>
where
    I2C: I2c,
{
    pub(crate) fn register_bit_flag_high(
        &mut self,
        address: u8,
        bitmask: u8,
    ) -> Result<bool, Error<I2C::Error>> {
        let data = self.read_register(address)?;
        Ok((data & bitmask) != 0)
    }

    pub(crate) fn set_register_bit_flag(
        &mut self,
        address: u8,
        bitmask: u8,
    ) -> Result<(), Error<I2C::Error>> {
        let data = self.read_register(address)?;
        if (data & bitmask) == 0 {
            self.write_register(address, data | bitmask)
        } else {
            Ok(())
        }
    }

    pub(crate) fn clear_register_bit_flag(
        &mut self,
        address: u8,
        bitmask: u8,
    ) -> Result<(), Error<I2C::Error>> {
        let data = self.read_register(address)?;
        if (data & bitmask) != 0 {
            self.write_register(address, data & !bitmask)
        } else {
            Ok(())
        }
    }

    pub(crate) fn write_register(
        &mut self,
        register: u8,
        data: u8,
    ) -> Result<(), Error<I2C::Error>> {
        let payload: [u8; 2] = [register, data];
        self.i2c.write(ADDR, &payload).map_err(Error::I2C)
    }

    pub(crate) fn read_register(&mut self, register: u8) -> Result<u8, Error<I2C::Error>> {
        let mut data = [0];
        self.i2c
            .write_read(ADDR, &[register], &mut data)
            .map_err(Error::I2C)
            .and(Ok(data[0]))
    }
}
