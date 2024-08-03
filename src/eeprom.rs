use crate::{Error, Register, Rv3029, ADDR};
use embedded_hal::i2c::I2c;

const EEPROM_BYTE_COUNT: usize = (Register::EEPROM_END - Register::EEPROM_BEGIN + 1) as usize;

impl<I2C> Rv3029<I2C>
where
    I2C: I2c,
{
    /// Read a data array from the user EEPROM starting at the given offset.
    ///
    /// There is a total of 2 bytes of user EEPROM available so the valid ranges for
    /// the parameters are: `address_offset`: [0-1] and `data` array length: [0-2].
    ///
    /// Will return an `Error::InvalidInputData` if attempting to access a position not
    /// available or if attempting to read too much data.
    pub fn read_eeprom(
        &mut self,
        address_offset: u8,
        data: &mut [u8],
    ) -> Result<(), Error<I2C::Error>> {
        if data.is_empty() {
            return Ok(());
        }
        self.check_eeprom_parameters(address_offset, &data)?;
        self.i2c
            .write_read(
                ADDR,
                &[Register::EEPROM_BEGIN + address_offset],
                &mut data[..],
            )
            .map_err(Error::I2C)
    }

    /// Write a data array to the user EEPROM starting at the given offset.
    ///
    /// There is a total of 2 bytes of user EEPROM available so the valid ranges for
    /// the parameters are: `address_offset`: [0-1] and `data` array length: [0-2].
    ///
    /// Will return an `Error::InvalidInputData` if attempting to access a position not
    /// available or if attempting to write too much data.
    pub fn write_eeprom(
        &mut self,
        address_offset: u8,
        data: &[u8],
    ) -> Result<(), Error<I2C::Error>> {
        if data.is_empty() {
            return Ok(());
        }
        self.check_eeprom_parameters(address_offset, &data)?;
        let mut payload = [0; EEPROM_BYTE_COUNT + 1];
        payload[0] = Register::EEPROM_BEGIN + address_offset;
        payload[1..=data.len()].copy_from_slice(&data);
        self.i2c
            .write(ADDR, &payload[..=data.len()])
            .map_err(Error::I2C)
    }

    fn check_eeprom_parameters(
        &self,
        address_offset: u8,
        data: &[u8],
    ) -> Result<(), Error<I2C::Error>> {
        if address_offset >= EEPROM_BYTE_COUNT as u8
            || (address_offset as usize + data.len()) > EEPROM_BYTE_COUNT
        {
            return Err(Error::InvalidInputData);
        }
        Ok(())
    }
}
