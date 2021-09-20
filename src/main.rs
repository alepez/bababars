mod bars;
mod config;
mod signals;

use bars::Bars;
use config::Config;
use std::{thread, time};

fn main() {
    let config = r#"
[signals.A]
name = "A"
unit = "rad"
range = { min = 0.0, max =  6.283185307179586 }

[signals.B]
name = "B"
unit = "deg"
range = { min = 0.0, max =  360.0 }
"#;

    let config: Config = config.parse().unwrap();

    let mut bars = Bars::from(config.signals.clone());

    let mut x = 0.0;
    let mut y = 0.0;

    loop {
        x += 6.283185307179586 / 100.0;
        y += 360.0 / 100.0;

        print!("\x1B[2J\x1B[1;1H");

        bars.update("A".into(), x);
        bars.update("B".into(), y);
        println!("{}", &bars);

        thread::sleep(time::Duration::from_millis(100));
    }
}
