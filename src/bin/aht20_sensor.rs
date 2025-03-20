use grove_rs::AHT20;
use std::thread::sleep;
use std::time::Duration;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut sensor = AHT20::new()?;

    loop {
        match sensor.get_sensor() {
            Ok((humidity, temperature)) => {
                println!("Humididty: {:.2}%, Temperature: {:.2}°C", humidity, temperature);
            }
            Err(e) => {
                eprintln!("Error reading sensor: {:?}", e);
            }
        }

        sleep(Duration::from_secs(2));
    }
}
