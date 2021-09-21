mod bars;
mod config;
mod signals;

use bars::Bars;
use config::Config;
use std::{thread, time};

struct InputRecord {
    key: String,
    value: f64,
}

struct InputStream {
    key_index: usize,
    value: f64,
}

impl InputStream {
    fn new() -> Self {
        Self {
            key_index: 0,
            value: 0.0,
        }
    }
}

impl Iterator for InputStream {
    type Item = InputRecord;

    fn next(&mut self) -> Option<Self::Item> {
        let key = if self.key_index == 0 { "A" } else { "B" };
        self.value += 1.0;
        self.key_index = if self.key_index == 0 { 1 } else { 0 };

        Some(InputRecord {
            key: key.into(),
            value: self.value,
        })
    }
}

fn main() {
    let config = r#"
[signals.A]
name = "A"
unit = "deg"
range = { min = 0.0, max =  360.0 }

[signals.B]
name = "B"
unit = "deg"
range = { min = 0.0, max =  360.0 }
"#;

    let config: Config = config.parse().unwrap();

    let mut bars = Bars::from(config.signals.clone());

    let input_stream = InputStream::new();

    for x in input_stream {
        print!("\x1B[2J\x1B[1;1H");
        bars.update(x.key, x.value);
        println!("{}", &bars);
        thread::sleep(time::Duration::from_millis(100));
    }
}
