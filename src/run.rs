use crate::{BitFlags, Error, Register, Rv3029};
use embedded_hal::i2c::blocking::I2c;

impl<I2C> Rv3029<I2C>
where
    I2C: I2c,
{
    /// Read if the clock is running.
    pub fn running(&mut self) -> Result<bool, Error<I2C::Error>> {
        self.register_bit_flag_high(Register::SECONDS, BitFlags::CH)
    }

    /// Set the clock to run (default on power-on).
    /// (Does not alter the device register if already running).
    pub fn set_running(&mut self) -> Result<(), Error<I2C::Error>> {
        self.set_register_bit_flag(Register::SECONDS, BitFlags::CH)
    }

    /// Halt the clock.
    /// (Does not alter the device register if already halted).
    pub fn halt(&mut self) -> Result<(), Error<I2C::Error>> {
        self.clear_register_bit_flag(Register::SECONDS, BitFlags::CH)
    }
}
