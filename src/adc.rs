use linux_embedded_hal::I2cdev;
use i2cdev::linux::LinuxI2CError;
use i2cdev::core::I2CDevice;


const ADC_ADDRESS: u8 = 0x08;


/*
Grove Base Hat for RPI I2C Registers
0x10 ~ 0x17: ADC raw data
0x20 ~ 0x27: input voltage
0x29: output voltage (Grove power supply voltage)
0x30 ~ 0x37: input voltage / output voltage
*/

pub struct ADC {
    device: I2cdev,
}

impl ADC {
    pub fn new() -> Result<Self, LinuxI2CError> {
        let device = I2cdev::new("/dev/i2c-1")?;
        Ok(ADC{device})
    }

    pub fn read_register(&mut self, register: u8) -> Result<u16, LinuxI2CError> {
        let mut buf = [0u8; 2];
        self.device.set_slave_address(ADC_ADDRESS as u16)?;
        self.device.write(&[register])?;
        self.device.read(&mut buf)?;

        //Debug-Print
        //println!("Raw bytes: {:?}", buf);
        Ok(u16::from_le_bytes(buf))
    }

    // read voltag/output voltage (%)
    pub fn read(&mut self, channel: u8) -> Result<u16, LinuxI2CError>{
        let addr = 0x30 +channel;
        self.read_register(addr)
    }

    // read input voltage (mV)
    pub fn read_voltage(&mut self, channel: u8) -> Result<u16, LinuxI2CError> {
        let addr = 0x20 + channel;
        self.read_register(addr)
    }

    pub fn read_raw(&mut self, channel: u8) -> Result<u16, LinuxI2CError>{
        let addr = 0x10 + channel;
        self.read_register(addr)
        
    }

    
}
