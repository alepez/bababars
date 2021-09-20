use serde::{Deserialize, Serialize};
use std::{collections::BTreeMap, str::FromStr};

type Real = f64;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
struct Range {
    pub min: Real,
    pub max: Real,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
struct Signal {
    pub name: String,
    pub unit: String,
    pub range: Range,
}

type SignalCode = String;

type Signals = BTreeMap<SignalCode, Signal>;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
struct Config {
    signals: Signals,
}

impl FromStr for Signal {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tokens: Vec<_> = s.split_whitespace().collect();
        if tokens.len() == 4 {
            let s = Signal {
                name: tokens[0].to_string(),
                unit: tokens[1].to_string(),
                range: Range {
                    min: tokens[2].parse().unwrap(),
                    max: tokens[3].parse().unwrap(),
                },
            };
            Ok(s)
        } else {
            Err(())
        }
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_signal() {
        let s = "name = \"A\"\nunit = \"rad\"\n\n[range]\nmin = 0.0\nmax = 6.283185307179586\n";
        let _s: Signal = toml::from_str(s).unwrap();
    }

    #[test]
    fn test_config_signals() {
        let s = r#"
[signals.A]
name = "A"
unit = "rad"
range = { min = 0.0, max =  6.283185307179586 }

[signals.B]
name = "A"
unit = "rad"
range = { min = 0.0, max =  6.283185307179586 }
            "#;
        let _s: Config = toml::from_str(s).unwrap();
    }
}
