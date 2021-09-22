use crate::signals::Signals;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub(crate) struct Render {
    pub width: usize,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub(crate) struct Config {
    pub signals: Signals,
    pub render: Render,
}

impl FromStr for Config {
    type Err = toml::de::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        toml::from_str(s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_signals() {
        let s = r#"
[render]
width = 100

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
