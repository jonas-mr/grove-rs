use i2cdev::linux::LinuxI2CError;

use crate::adc::ADC;

pub struct GroveMoistureSensor {
    channel: u8,
    adc: ADC
}

impl GroveMoistureSensor {
    pub fn new(channel: u8) -> Result<Self, LinuxI2CError>{
        let adc = ADC::new()?;
        Ok(GroveMoistureSensor { channel, adc })
    }
     pub fn moisture(&mut self) -> Result<u16, LinuxI2CError>{
         self.adc.read(self.channel)
     }

     pub fn debug(&mut self) -> Result<(), LinuxI2CError> {
         let input_voltage: f32 = self.adc.read_voltage(self.channel)?.into();
         let ouput_voltage: f32 = self.adc.read_voltage(9)?.into();
            
         println!("in_voltage: {}, out_voltage: {}", input_voltage, ouput_voltage);

         let calculated_moisture:f32 = input_voltage / ouput_voltage;

         let measured_moisture = self.moisture()?;
         println!("Moisture berechnet: {}, Moisture gemessen: {}", calculated_moisture, measured_moisture);
        Ok(())
     }
    
}
