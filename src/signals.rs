use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

pub(crate) type Real = f64;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub(crate) struct Range {
    pub min: Real,
    pub max: Real,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub(crate) struct ConversionExpr(String);

fn default_conversion() -> ConversionExpr {
    ConversionExpr("x".to_string())
}

impl From<&str> for ConversionExpr {
    fn from(s: &str) -> Self {
        ConversionExpr(s.into())
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub(crate) struct Signal {
    pub name: String,
    pub unit: String,
    pub range: Range,
    #[serde(default = "default_conversion")]
    pub conversion: ConversionExpr,
}

pub(crate) type SignalCode = String;

pub(crate) type Signals = BTreeMap<SignalCode, Signal>;
