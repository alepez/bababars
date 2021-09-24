mod bars;
mod config;
mod input;
mod signals;

use crate::bars::TextBars;
use crate::config::Config;
use crate::input::InputStreamStdin;
use std::fs::read_to_string;

fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
}

const DEFAULT_CONFIG_FILE: &'static str = "bababars.toml";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = read_to_string(DEFAULT_CONFIG_FILE)?;
    let config: Config = config.parse()?;

    let mut bars = TextBars::from(config);

    let input_stream = InputStreamStdin::new();

    for x in input_stream.filter_map(|x| x) {
        bars.update(x.key, x.value);
        clear_screen();
        println!("{}", &bars);
    }

    Ok(())
}
