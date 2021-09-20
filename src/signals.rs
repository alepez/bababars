use serde::{Deserialize, Serialize};
use std::{collections::BTreeMap, str::FromStr};

pub(crate) type Real = f64;

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
