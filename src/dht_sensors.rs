use rppal::gpio::{Gpio, Mode};
use std::{thread::sleep, time::Duration};

pub const PULSES_CNT:usize = 41;
pub const MAX_CNT:u32 = 320;


#[derive(Debug)]
pub enum DhtType{
    DHT11,
    DHT22
}

pub struct DHT {
    pin: u8,
    pub dht_type: DhtType,
    last_temp: f32,
    last_humi: f32
}

impl DHT {
    pub fn new(dht_type:DhtType, pin: u8) -> Self {
        DHT{
            pin,
            dht_type, 
            last_temp: 0.0,
            last_humi: 0.0,
        }
    }

    pub fn read(&mut self, gpio: &Gpio) -> Option<(f32, f32)> {
        // 15 retries
        for _ in 0..15 {
            if let Some((humi, temp)) = self._read(gpio) {
                self.last_temp = temp;
                self.last_humi = humi;
                return Some((humi, temp));
            }
        }
        return Some((self.last_humi, self.last_temp))
    }

    fn _read(&self, gpio: &Gpio) -> Option<(f32, f32)> {
        let mut pin = gpio.get(self.pin).unwrap().into_io(Mode::Output);
        pin.set_high();
        sleep(Duration::from_millis(200));
        pin.set_low();
        sleep(Duration::from_millis(18));

        pin.set_mode(Mode::Input);


        let mut count = 0;
        while pin.is_high() {
            count += 1;
            if count > MAX_CNT {
                return None;
            }

        }

        let mut pulse_cnt = [0; 2 * PULSES_CNT];
        for i in (0..2*PULSES_CNT).step_by(2){
            while pin.is_low() {
                pulse_cnt[i] += 1;
                if pulse_cnt[i] > MAX_CNT {
                    return None;
                }


            }
            while pin.is_high() {
                pulse_cnt[i+1] += 1;
                if pulse_cnt[i+1] > MAX_CNT {
                    return None;
                }

            }
        }

        let total_cnt: u32 = pulse_cnt.iter().skip(2).step_by(2).sum();
        let average_cnt: u32 = total_cnt/(PULSES_CNT-1) as u32;
        let mut data = String::new();
        for i in (3..2*PULSES_CNT).step_by(2) {
            if pulse_cnt[i] > average_cnt{
                data.push('1');
            }
            else
            {
                data.push('0');
            }
        }

        let data_0 = u8::from_str_radix(&data[0..8], 2).unwrap();
        let data_1 = u8::from_str_radix(&data[8..16], 2).unwrap();
        let data_2 = u8::from_str_radix(&data[16..24], 2).unwrap();
        let data_3 = u8::from_str_radix(&data[24..32], 2).unwrap();
        let data_4 = u8::from_str_radix(&data[32..40], 2).unwrap();

        if data_4 != ((data_0 + data_1 + data_2 + data_3)&0xFF){
            return None;
        }

        match self.dht_type {
            DhtType::DHT11 => {
                let humi = data_0 as f32;
                let temp = data_2 as f32;
                Some((humi, temp))
            }
            DhtType::DHT22 => {
                let humi = (u16::from_str_radix(&data[0..16], 2).unwrap() as f32) * 0.1;
                let temp = (u16::from_str_radix(&data[17..32], 2).unwrap() as f32) * 0.2*(0.5 -(data[16..17].parse::<f32>().unwrap()));
                Some((humi, temp))
            }
        }



    }

}
