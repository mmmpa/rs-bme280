use crate::*;
use i2cdev::core::I2CDevice;
use i2cdev::linux::{LinuxI2CDevice, LinuxI2CError};

pub struct Bme280Client {
    core: Bme280CoreClient,
}

impl Bme280Client {
    pub fn new(i2c_cli: LinuxI2CDevice) -> Self {
        let core = Bme280CoreClient { i2c_cli };
        Self { core }
    }
}

pub struct Bme280CoreClient {
    i2c_cli: LinuxI2CDevice,
}

impl Bme280 for Bme280Client {
    type Bme280Core = Bme280CoreClient;

    fn core(&self) -> &Self::Bme280Core {
        &self.core
    }

    fn core_mut(&mut self) -> &mut Self::Bme280Core {
        &mut self.core
    }
}

impl I2c for Bme280CoreClient {
    fn write_i2c_block_data(&mut self, reg: RegisterAddress, data: &[u8]) -> Bme280Result<()> {
        self.i2c_cli.smbus_write_block_data(reg as u8, data)?;
        Ok(())
    }

    fn write_byte_data(&mut self, reg: RegisterAddress, data: u8) -> Bme280Result<()> {
        self.i2c_cli.smbus_write_byte_data(reg as u8, data)?;
        Ok(())
    }

    fn read_byte_data(&mut self, reg: RegisterAddress) -> Bme280Result<u8> {
        let re = self.i2c_cli.smbus_read_byte_data(reg as u8)?;
        Ok(re)
    }

    fn read_i2c_block_data(&mut self, reg: RegisterAddress, data: &mut [u8]) -> Bme280Result<()> {
        let re = self
            .i2c_cli
            .smbus_read_i2c_block_data(reg as u8, data.len() as u8)?;
        for i in 0..data.len() {
            data[i] = re[i];
        }
        Ok(())
    }
}

impl Bme280Core for Bme280CoreClient {
    type I2c = Bme280CoreClient;

    fn i2c(&mut self) -> &mut Self::I2c {
        self
    }
}

impl From<LinuxI2CError> for Bme280Error {
    fn from(e: LinuxI2CError) -> Self {
        Self::I2cError(e.to_string())
    }
}
