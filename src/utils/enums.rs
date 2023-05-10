use std::fmt;

use clap::{ValueEnum, builder::PossibleValue};

#[derive(Debug, Clone)]
pub enum FileType {
    CSV,
    JSON,
}

impl Default for FileType {
    fn default() -> Self {
        FileType::CSV
    }
}

impl fmt::Display for FileType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            FileType::CSV => write!(f, "csv"),
            FileType::JSON => write!(f, "json"),
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Mode {
    MASK,
    ENCRYPT,
    DECRYPT,
}

impl Default for Mode {
    fn default() -> Self {
        Mode::MASK
    }
}

// Can also be derived with feature flag `derive`
impl ValueEnum for Mode {
    fn value_variants<'a>() -> &'a [Self] {
        &[Mode::MASK, Mode::ENCRYPT, Mode::DECRYPT]
    }

    fn to_possible_value<'a>(&self) -> Option<PossibleValue> {
        Some(match self {
            Mode::MASK => PossibleValue::new("mask").help("Mask the data by *"),
            Mode::ENCRYPT => PossibleValue::new("encrypt").help("Encrypt the data with provided KEY"),
            Mode::DECRYPT => PossibleValue::new("decrypt").help("Decrypt the data with provided KEY"),
        })
    }
}

impl std::fmt::Display for Mode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.to_possible_value()
            .expect("no values are skipped")
            .get_name()
            .fmt(f)
    }
}

impl std::str::FromStr for Mode {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        for variant in Self::value_variants() {
            if variant.to_possible_value().unwrap().matches(s, false) {
                return Ok(*variant);
            }
        }
        Err(format!("invalid variant: {}", s))
    }
}

impl std::fmt::Debug for Mode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MASK => write!(f, "MASK"),
            Self::ENCRYPT => write!(f, "ENCRYPT"),
            Self::DECRYPT => write!(f, "DECRYPT"),
        }
    }
}
