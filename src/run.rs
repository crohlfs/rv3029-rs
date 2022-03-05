use crate::{BitFlags, Error, Register, Rv3029};
use embedded_hal::blocking::i2c::{Write, WriteRead};

impl<I2C, E> Rv3029<I2C>
where
    I2C: Write<Error = E> + WriteRead<Error = E>,
{
    /// Read if the clock is running.
    pub fn running(&mut self) -> Result<bool, Error<E>> {
        self.register_bit_flag_high(Register::SECONDS, BitFlags::CH)
    }

    /// Set the clock to run (default on power-on).
    /// (Does not alter the device register if already running).
    pub fn set_running(&mut self) -> Result<(), Error<E>> {
        self.set_register_bit_flag(Register::SECONDS, BitFlags::CH)
    }

    /// Halt the clock.
    /// (Does not alter the device register if already halted).
    pub fn halt(&mut self) -> Result<(), Error<E>> {
        self.clear_register_bit_flag(Register::SECONDS, BitFlags::CH)
    }
}
