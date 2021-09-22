mod bars;
mod config;
mod signals;

use bars::Bars;
use config::Config;
use std::{fs::read_to_string, io::Stdin};

#[derive(Debug)]
struct InputRecord {
    key: String,
    value: f64,
}

impl std::str::FromStr for InputRecord {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (key, value) = s.split_once(' ').ok_or(())?;
        let value = value.trim();
        let value = value.parse().ok().ok_or(())?;
        let key = key.into();

        Ok(InputRecord { key, value })
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
    type Item = Option<InputRecord>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut line = String::new();
        self.stdin.read_line(&mut line).ok()?;

        let record = line.parse().ok();

        Some(record)
    }
}

fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
}

const DEFAULT_CONFIG_FILE: &'static str = "bababars.toml";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = read_to_string(DEFAULT_CONFIG_FILE)?;
    let config: Config = config.parse()?;

    let mut bars = Bars::from(config);

    let input_stream = InputStreamStdin::new();

    for x in input_stream.filter_map(|x| x) {
        bars.update(x.key, x.value);
        clear_screen();
        println!("{}", &bars);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

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

    #[test]
    fn test_input_mock() {
        use std::fmt::Write;

        let config = r#"
[render]
width = 100

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

        let mut bars = Bars::from(config);

        let input_stream = InputStreamMock::new();

        let mut s = String::new();

        for x in input_stream.take(10) {
            bars.update(x.key, x.value);
            write!(s, "{}", &bars).unwrap();
        }

        assert!(!s.is_empty());
    }
}
