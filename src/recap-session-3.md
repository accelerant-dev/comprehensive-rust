# Recap / Warm Up

Yesterday, we covered a lot of ground. We started by talking about strings, and
rounded that out with lots of information about other data types.

As a bit of a warm up, let's start by watching me implement a utility to read in
CSV data.

We'll extend it together once that's done.

<details>

```rust
// csv = { version = "1" }
// serde = { version = "1", features = ["derive"] }

use std::{error::Error, io::{self, Read}};
use serde::Deserialize;

static DATA: &str = "timestamp,sensor_id,reading
1698313269,1,0.11
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

fn main() -> Result<(), Box<dyn Error>> {
    let mut r = csv::Reader::from_reader(DATA.as_bytes());

    for result in r.deserialize() {
        let record: SensorReading = result?;
        println!("{:?}", record);
    }

    Ok(())
}
```
</details>