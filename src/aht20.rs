/*
 * Code ported from: https://github.com/Seeed-Studio/Seeed_Arduino_AHT20/blob/master/src/ATH20.cpp
 *
 */

use linux_embedded_hal::I2cdev;
use i2cdev::linux::LinuxI2CError;
use i2cdev::core::I2CDevice;

const AHT20_ADDRESS: u8 = 0x38;

pub struct AHT20{
    device: I2cdev,
}

impl AHT20 {
    pub fn new() -> Result<Self, LinuxI2CError> {
        let device = I2cdev::new("/dev/i2c-1")?; //Standard-I2C-Bus on Raspi
        let mut sensor = AHT20 {device};
        sensor.begin()?;
        Ok(sensor)
    }

    fn begin(&mut self) -> Result<(), LinuxI2CError> {
        self.device.set_slave_address(AHT20_ADDRESS as u16)?;
        self.device.write(&[0xBE])?;
        Ok(())
    }
    fn start_sensor(&mut self) -> Result<bool, LinuxI2CError> {
        self.device.set_slave_address(AHT20_ADDRESS as u16)?;
        self.device.write(&[0xAC, 0x33, 0x00])?; //start measuring
        let mut retries = 10;
        while retries > 0 {
            let mut buf = [0u8; 1];
            self.device.read(&mut buf)?;

            if buf[0] & 0x80 == 0 {
                return Ok(true); //sensor ready
            }

            retries -= 1;
            std::thread::sleep(std::time::Duration::from_millis(20));
            
        }
        Ok(false) //timeout
    }

    pub fn get_sensor(&mut self) -> Result<(f32, f32), LinuxI2CError> {
        self.start_sensor()?;

        let mut buf = [0u8; 6];
        self.device.read(&mut buf)?;

        if buf[0] & 0x80 != 0 {
            return Err(LinuxI2CError::Io(std::io::Error::new(
                        std::io::ErrorKind::Other, 
                        "Sensor busy",
                        )));
        }
        let humi_raw = ((buf[1] as u32) << 12) | ((buf[2] as u32) << 4) | ((buf[3] as u32) >> 4);
        let temp_raw = (((buf[3] as u32) & 0x0F) <<16) | ((buf[4] as u32) << 8) | (buf[5] as u32);

        let humidity = (humi_raw as f32) / 1048576.0 * 100.0;
        let temperature = (temp_raw as f32) / 1048576.0 * 200.0 - 50.0;

        Ok((humidity, temperature))
        
    }

    pub fn get_temperature(&mut self) -> Result<f32, LinuxI2CError> {
        let (_, temperature) = self.get_sensor()?;
        Ok(temperature)
    }
    pub fn get_humidity(&mut self) -> Result<f32, LinuxI2CError> {
        let (humidity,_) = self.get_sensor()?;
        Ok(humidity)
    }
    
}
