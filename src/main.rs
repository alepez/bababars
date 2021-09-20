mod config;

fn main() {
    let config = r#"
[signals.A]
name = "A"
unit = "rad"
range = { min = 0.0, max =  6.283185307179586 }

[signals.B]
name = "A"
unit = "rad"
range = { min = 0.0, max =  6.283185307179586 }
"#;

    let config: config::Config = config.parse().unwrap();

    dbg!(&config);
}
