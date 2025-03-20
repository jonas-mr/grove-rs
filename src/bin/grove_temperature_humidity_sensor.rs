use grove_rs::{DhtType, DHT};
use rppal::gpio::Gpio;
use std::{thread::sleep, time::Duration};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: {} dht_type pin", args[0]);
        std::process::exit(1);
    }
    let dht_type = match args[1].as_str() {
        "11" => DhtType::DHT11,
        "22" => DhtType::DHT22,
        _ => {
            eprintln!("ERROR: Please use 11|22 as dht_type.");
            std::process::exit(1);

        }
    };

    let pin = args[2].parse::<u8>().unwrap();
    let gpio = Gpio::new().unwrap();
    let mut sensor = DHT::new(dht_type, pin);
    loop {
        if let Some((humi, temp)) = sensor.read(&gpio) {
            println!("DHT{:?}, humidity {:.1}%, temperature {:.1}*", sensor.dht_type, humi, temp);
        }
        else {
            println!("DHT{:?}, humidity & temperature: None", sensor.dht_type);
        }

        sleep(Duration::from_secs(1));
    }

} 
