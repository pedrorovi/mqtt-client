use mpu6050::*;

use esp_idf_hal::i2c::*;

pub mod i2c_controller;
use i2c_controller::I2cController;

pub struct Mpu6050Controller<'a> {
    driver: Mpu6050<I2cDriver<'a>>,
}

pub struct Mpu6050Data {
    pub acceleration: (f32, f32, f32),
    pub rotation: (f32, f32, f32),
    pub angles: (f32, f32),
    pub temperature: f32,
}

impl<'a> Mpu6050Controller<'a> {
    pub fn new(i2c_cont: I2cController<'a>) -> Result<Self, Mpu6050Error<I2cError>> {
        log::info!("Initializing MPU6050...");
        let mut driver = Mpu6050::new(i2c_cont.driver);
        let mut delay = esp_idf_hal::delay::Ets;
        driver.init(&mut delay)?;
        driver.set_accel_range(mpu6050::device::AccelRange::G2)?;
        driver.set_gyro_range(mpu6050::device::GyroRange::D250)?;

        log::info!("MPU6050 initialized successfully");
        Ok(Mpu6050Controller { driver })
    }

    pub fn get_acceleration(&mut self) -> Result<(f32, f32, f32), Mpu6050Error<I2cError>> {
        let accel = self.driver.get_acc()?;
        Ok((accel[0], accel[1], accel[2]))
    }

    pub fn get_rotation(&mut self) -> Result<(f32, f32, f32), Mpu6050Error<I2cError>> {
        let rotation = self.driver.get_gyro()?;
        Ok((rotation[0], rotation[1], rotation[2]))
    }

    pub fn get_angles(&mut self) -> Result<(f32, f32), Mpu6050Error<I2cError>> {
        let angles = self.driver.get_acc_angles()?;
        Ok((angles[0], angles[1]))
    }

    pub fn get_temperature(&mut self) -> Result<f32, Mpu6050Error<I2cError>> {
        Ok(self.driver.get_temp()?)
    }

    pub fn get(&mut self) -> Result<Mpu6050Data, Mpu6050Error<I2cError>> {
        Ok(Mpu6050Data {
            acceleration: self.get_acceleration()?,
            rotation: self.get_rotation()?,
            angles: self.get_angles()?,
            temperature: self.get_temperature()?,
        })
    }
}
