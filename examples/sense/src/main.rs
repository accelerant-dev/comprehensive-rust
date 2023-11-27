use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
struct SensorReading {
    sensor_id: u32,
    timestamp: u32,
    reading: f32,
}

fn main() {
    // string literals are not "String" types
    let source_data = "timestamp,sensor_id,reading
    1698313269,1,0.11
    1698313270,2,4.11
    1698313296,1,0.12

    # this is a comment
    1698313297,1,0.11
    1698313381,2,4.10
    1698313524,2,4.09
    ";

    let mut data: HashMap<u32, Vec<f32>> = HashMap::<u32, Vec<f32>>::new();

    let mut r = csv::Reader::from_reader(source_data.as_bytes());

    let readings = r.deserialize::<SensorReading>();

    for reading in readings {
        if let Ok(valid) = reading {
            if data.contains_key(&valid.sensor_id) {
                let v: &mut Vec<f32> = data.get_mut(&valid.sensor_id).unwrap();
                v.push(valid.reading);
            } else {
                data.insert(valid.sensor_id, vec![valid.reading]);
            }
        }
    }

    // for record in r.records() {
    //     if let Ok(valid_record) = record {
    //             let _timestamp = fields.get(1);
    //             let sensor_id = fi>elds.get(2);
    //             let reading = fields.get(3);

    //             // ...
    //         println!("{valid_record:?}");
    //     }
    // }

    // for (i, line) in source_data.lines().enumerate() {
    //     if i == 0 {
    //         // skip the header
    //         continue;
    //     }

    //     let line = line.trim();
    //     if line.is_empty() {
    //         continue;
    //     }

    //     if line.starts_with('#') {
    //         continue;
    //     }

    //     let mut fields = line
    //         .trim()
    //         .split(',');

    //     let _timestamp = fields.next();
    //     let sensor_id = fields.next();
    //     let reading = fields.next();

    //     // let timestamp: u32 = match timestamp {
    //     //     None => continue,
    //     //     Some(ts_raw) => {
    //     //         match ts_raw.parse() {
    //     //             Ok(ts) => ts,
    //     //             Err(_) => continue,
    //     //         }
    //     //     }
    //     // };

    //     // let ts = if let Some(ts_raw) = timestamp {
    //     //     if let Ok(ts) = ts_raw.parse::<u32>() {
    //     //         ts
    //     //     } else {
    //     //         continue;
    //     //     }
    //     // } else {
    //     //     continue;
    //     // };

    //     let sensor_id: u32 = match sensor_id {
    //         None => continue,
    //         Some(s_raw) => {
    //             match s_raw.parse() {
    //                 Ok(s) => s,
    //                 Err(_) => continue,
    //             }
    //         }
    //     };

    //     let reading: f32 = match reading {
    //         None => continue,
    //         Some(reading_raw) => {
    //             match reading_raw.parse() {
    //                 Ok(s) => s,
    //                 Err(_) => continue,
    //             }
    //         }
    //     };

    //     if data.contains_key(&sensor_id) {
    //         let v: &mut Vec<f32> = data.get_mut(&sensor_id).unwrap();
    //         v.push(reading);
    //     } else {
    // for (i, line) in source_data.lines().enumerate() {
    //     if i == 0 {
    //         // skip the header
    //         continue;
    //     }

    //     let line = line.trim();
    //     if line.is_empty() {
    //         continue;
    //     }

    //     if line.starts_with('#') {
    //         continue;
    //     }

    //     let mut fields = line
    //         .trim()
    //         .split(',');

    //     let _timestamp = fields.next();
    //     let sensor_id = fields.next();
    //     let reading = fields.next();

    //     // let timestamp: u32 = match timestamp {
    //     //     None => continue,
    //     //     Some(ts_raw) => {
    //     //         match ts_raw.parse() {
    //     //             Ok(ts) => ts,
    //     //             Err(_) => continue,
    //     //         }
    //     //     }
    //     // };

    //     // let ts = if let Some(ts_raw) = timestamp {
    //     //     if let Ok(ts) = ts_raw.parse::<u32>() {
    //     //         ts
    //     //     } else {
    //     //         continue;
    //     //     }
    //     // } else {
    //     //     continue;
    //     // };

    //     let sensor_id: u32 = match sensor_id {
    //         None => continue,
    //         Some(s_raw) => {
    //             match s_raw.parse() {
    //                 Ok(s) => s,
    //                 Err(_) => continue,
    //             }
    //         }
    //     };

    //     let reading: f3
    //         data.insert(sensor_id, vec![reading]);
    //     };
    // }

    for (sensor, readings) in data.iter() {
        let mut min = f32::MAX;
        let mut max = 0.0;
        let mut sum = 0.0;

        for reading in readings {
            if *reading < min {
                min = *reading;
            }

            if *reading > max {
                max = *reading;
            }

            sum += *reading;
        }

        let avg = sum / readings.len() as f32;

        println!("{sensor}: {min} - {max} ({avg})");
    }
}
