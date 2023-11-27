use std::error::Error;
use serde::Deserialize;

static DATA: &str = "timestamp,sensor_id,reading
1698313269,1,0.
1698313270,2,4.11
1698313296,1,0.12
1698313297,1,0.11
1698313381,2,4.10
1698313524,2,4.09
";

#[derive(Debug, Deserialize)]
pub struct SensorReading {
    timestamp: u64,
    sensor_id: String,
    reading: f64,
}

fn main() -> Result<(), ()> {
    let mut r = csv::Reader::from_reader(DATA.as_bytes());

    for result in r.deserialize::<SensorReading>() {
        if let Ok(reading) = result {
            println!("{:?}", reading);
        }

    Ok(())
}