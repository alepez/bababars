use std::collections::BTreeMap;

use crate::signals::{Real, Signal, SignalCode, Signals};
use crate::Config;

#[derive(Debug)]
pub(crate) struct Bar {
    signal: Signal,
    value: Option<Real>,
    width: usize,
}

impl Bar {
    fn update(&mut self, x: Real) {
        self.value = Some(x);
    }

    fn calculate_fill_width(&self) -> usize {
        if let Some(x) = self.value {
            let width = self.width as Real;
            let range_width = self.signal.range.max - self.signal.range.min;
            let x = x as Real;
            let y = (x / range_width) * width;
            y.round() as usize
        } else {
            0
        }
    }
}

impl std::fmt::Display for Bar {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let fill_width = self.calculate_fill_width();
        let empty_width = self.width - fill_width;
        write!(f, "[")?;
        write!(f, "{:=^1$}", "", fill_width)?;
        write!(f, "{: ^1$}", "", empty_width)?;
        write!(f, "] {}", &self.signal.name)?;

        if let Some(x) = self.value {
            write!(f, " {:10.3} ", x)?;
        }

        write!(f, "({})", &self.signal.unit)
    }
}

#[derive(Debug, Default)]
pub(crate) struct Bars(pub BTreeMap<SignalCode, Bar>);

impl Bars {
    pub(crate) fn update(&mut self, code: String, value: Real) {
        self.0.entry(code).and_modify(|x| x.value = Some(value));
    }
}

impl From<Config> for Bars {
    fn from(config: Config) -> Self {
        let mut bars = Bars::default();

        let Config { signals, render } = config;

        for s in signals {
            let bar = Bar {
                signal: s.1.clone(),
                value: None,
                width: render.width,
            };
            bars.0.insert(s.0, bar);
        }

        bars
    }
}

impl std::fmt::Display for Bars {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (_, bar) in &self.0 {
            write!(f, "{}\n", bar)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::{config::Render, signals::Range};

    use super::*;

    #[test]
    fn test_bar_fill() {
        let signal = Signal {
            name: "A".into(),
            unit: "rad".into(),
            range: Range {
                min: 0.0,
                max: 6.283185307179586,
            },
        };

        let mut bar = Bar {
            signal,
            value: Some(0.0),
            width: 100,
        };

        let fill_width = bar.calculate_fill_width();

        assert_eq!(fill_width, 0);

        bar.update(3.14);

        let fill_width = bar.calculate_fill_width();

        assert_eq!(fill_width, 50);
    }

    #[test]
    fn test_bar_display() {
        let signal = Signal {
            name: "A".into(),
            unit: "rad".into(),
            range: Range {
                min: 0.0,
                max: 6.283185307179586,
            },
        };

        let mut bar = Bar {
            signal,
            value: Some(0.0),
            width: 100,
        };

        let bar_str = bar.to_string();

        assert_eq!(&bar_str, "[                                                                                                    ] A      0.000 (rad)");

        bar.update(3.14);

        let bar_str = bar.to_string();

        assert_eq!(&bar_str, "[==================================================                                                  ] A       0.000  (rad)");
    }

    #[test]
    fn test_bars_from_signals() {
        let mut signals = Signals::new();

        let a = Signal {
            name: "A".into(),
            unit: "rad".into(),
            range: Range {
                min: 0.0,
                max: 6.283185307179586,
            },
        };

        let b = Signal {
            name: "B".into(),
            unit: "deg".into(),
            range: Range {
                min: 0.0,
                max: 359.0,
            },
        };

        signals.insert("A".into(), a);
        signals.insert("B".into(), b);

        let render = Render { width: 100 };

        let config = Config { signals, render };

        let mut bars = Bars::from(config);

        bars.update("A".into(), 5.0);
        bars.update("B".into(), 100.0);
    }
}
