use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

pub(crate) type Real = f64;
pub(crate) type ConversionExpr = String;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub(crate) struct Range {
    pub min: Real,
    pub max: Real,
}

fn default_conversion() -> String {
    "x".to_string()
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
