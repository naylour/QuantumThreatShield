use serde::{Deserialize, Serialize};
use std::str::FromStr;

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
