use std::io::Stdin;

#[derive(Debug)]
pub(crate) struct InputRecord {
    pub(crate) key: String,
    pub(crate) value: f64,
}

impl std::str::FromStr for InputRecord {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (key, value) = s.split_once(' ').ok_or(())?;
        let value = value.trim();
        let value = value.parse().ok().ok_or(())?;
        let key = key.into();

        Ok(InputRecord { key, value })
    }
}

pub(crate) struct InputStreamStdin {
    stdin: Stdin,
}

impl InputStreamStdin {
    pub(crate) fn new() -> Self {
        let stdin = std::io::stdin();
        Self { stdin }
    }
}

impl Iterator for InputStreamStdin {
    type Item = Option<InputRecord>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut line = String::new();
        self.stdin.read_line(&mut line).ok()?;

        let record = line.parse().ok();

        Some(record)
    }
}
