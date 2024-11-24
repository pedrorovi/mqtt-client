use esp_idf_hal::gpio::*;
use esp_idf_hal::i2c::*;
use esp_idf_hal::peripheral::*;
use esp_idf_hal::units::FromValueType;

pub struct I2cController<'a> {
    pub driver: I2cDriver<'a>,
}

impl<'a> I2cController<'a> {
    pub fn new(
        i2c: impl Peripheral<P = I2C0> + 'a,
        sda: impl Peripheral<P = impl InputPin + OutputPin> + 'a,
        scl: impl Peripheral<P = impl InputPin + OutputPin> + 'a,
        baudrate: u32,
    ) -> Result<Self, I2cError> {
        let mut i2c_config = I2cConfig::new().baudrate(baudrate.kHz().into());
        let driver = I2cDriver::new(i2c, sda, scl, &mut i2c_config)?;
        Ok(I2cController { driver })
    }
}
