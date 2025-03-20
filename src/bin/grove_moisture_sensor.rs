use grove_rs::GroveMoistureSensor;
use std::{thread::sleep, time::Duration, u8};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} adc_channel [--debug]", args[0]);
        std::process::exit(1);
    }


    let channel = args[1].parse::<u8>()?;

    let debug_mode = args.iter().any(|arg| arg == "--debug");

    let mut sensor = GroveMoistureSensor::new(channel)?;

    if debug_mode{
        loop{
        sensor.debug()?;
        }
    }

    println!("Detecting moisture...");
    loop {
        let m = sensor.moisture()?;
        let result = if m < 300 {
            "Dry"
        } else if m < 600 {
            "Moist"
        } else {
            "Wet"
        };
        println!("Moisture value: {}, {}", m, result);
        sleep(Duration::from_secs(1));
    }
}
