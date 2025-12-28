use serde::{Deserialize, Serialize};
use std::{fmt, str::FromStr};

#[derive(Serialize, Deserialize, Debug)]
pub enum Mode {
    Prod,
    Dev,
}

impl FromStr for Mode {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "DEV" => Ok(Self::Dev),
            "PROD" => Ok(Self::Prod),
            _ => Err(format!("Invalid MODE: {}", s)),
        }
    }
}

impl fmt::Display for Mode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            Mode::Dev => write!(f, "Разработка"),
            Mode::Prod => write!(f, "Продакшн"),
        }
    }
}
