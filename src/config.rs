use serde::{Deserialize, Serialize};
use std::{collections::BTreeMap, str::FromStr};

type Real = f64;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub(crate) struct Range {
    pub min: Real,
    pub max: Real,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub(crate) struct Signal {
    pub name: String,
    pub unit: String,
    pub range: Range,
}

pub(crate) type SignalCode = String;

pub(crate) type Signals = BTreeMap<SignalCode, Signal>;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub(crate) struct Config {
    signals: Signals,
}

impl FromStr for Config {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let x = toml::from_str(s);
        if let Ok(x) = x {
            Ok(x)
        } else {
            Err(())
        }
    }
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
