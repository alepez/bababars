mod bars;
mod config;
mod signals;

use bars::Bars;
use config::Config;
use std::{
    io::{Stdin, StdinLock},
    thread, time,
};

struct InputRecord {
    key: String,
    value: f64,
}

struct InputStreamMock {
    key_index: usize,
    value: f64,
}

impl InputStreamMock {
    fn new() -> Self {
        Self {
            key_index: 0,
            value: 0.0,
        }
    }
}

impl Iterator for InputStreamMock {
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

struct InputStreamStdin {
    stdin: Stdin,
}

impl InputStreamStdin {
    fn new() -> Self {
        let stdin = std::io::stdin();
        Self { stdin }
    }
}

impl Iterator for InputStreamStdin {
    type Item = InputRecord;

    fn next(&mut self) -> Option<Self::Item> {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).ok()?;

        let (key, value) = line.split_once(' ')?;
        let value = value.trim();
        let value = value.parse().ok()?;
        let key = key.into();

        Some(InputRecord { key, value })
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

    let input_stream = InputStreamStdin::new();

    for x in input_stream {
        print!("\x1B[2J\x1B[1;1H");
        bars.update(x.key, x.value);
        println!("{}", &bars);
    }
}
