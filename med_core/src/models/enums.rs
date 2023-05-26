use std::fmt;

use clap::{builder::PossibleValue, ValueEnum};
use serde::Serialize;

#[derive(Debug, Clone, Serialize, Default, PartialEq)]
pub enum FileType {
    #[default]
    CSV,
    JSON,
}

impl fmt::Display for FileType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            FileType::CSV => write!(f, "csv"),
            FileType::JSON => write!(f, "json"),
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Default)]
pub enum Mode {
    #[default]
    MASK,
    ENCRYPT,
    DECRYPT,
}

// Can also be derived with feature flag `derive`
impl ValueEnum for Mode {
    fn value_variants<'a>() -> &'a [Self] {
        &[Mode::MASK, Mode::ENCRYPT, Mode::DECRYPT]
    }

    fn to_possible_value<'a>(&self) -> Option<PossibleValue> {
        Some(match self {
            Mode::MASK => PossibleValue::new("mask").help("Mask the data by *"),
            Mode::ENCRYPT => {
                PossibleValue::new("encrypt").help("Encrypt the data with provided KEY")
            }
            Mode::DECRYPT => {
                PossibleValue::new("decrypt").help("Decrypt the data with provided KEY")
            }
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

#[derive(Clone, Serialize, PartialEq)]
#[allow(dead_code)]
#[derive(Default)]
pub enum AppMode {
    #[default]
    CLI,
    SDK,
}

impl std::fmt::Debug for AppMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::CLI => write!(f, "cli"),
            Self::SDK => write!(f, "sdk"),
        }
    }
}

impl fmt::Display for AppMode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AppMode::CLI => write!(f, "cli"),
            AppMode::SDK => write!(f, "sdk"),
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Default)]
pub enum Standard {
    #[default]
    DES64 = 64,
    AES128 = 128,
    AES192 = 192,
    AES256 = 256,
}

// Can also be derived with feature flag `derive`
impl ValueEnum for Standard {
    fn value_variants<'a>() -> &'a [Self] {
        &[
            Standard::DES64,
            Standard::AES128,
            Standard::AES192,
            Standard::AES256,
        ]
    }

    fn to_possible_value<'a>(&self) -> Option<PossibleValue> {
        Some(match self {
            Standard::DES64 => PossibleValue::new("des64").help("DES standard 64"),
            Standard::AES128 => PossibleValue::new("aes128").help("AES standard 128"),
            Standard::AES192 => PossibleValue::new("aes192").help("AES standard 192"),
            Standard::AES256 => PossibleValue::new("aes256").help("AES standard 256"),
        })
    }
}

impl std::fmt::Display for Standard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.to_possible_value()
            .expect("no values are skipped")
            .get_name()
            .fmt(f)
    }
}

impl std::str::FromStr for Standard {
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

impl std::fmt::Debug for Standard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DES64 => write!(f, "DES64"),
            Self::AES128 => write!(f, "AES128"),
            Self::AES192 => write!(f, "AES192"),
            Self::AES256 => write!(f, "AES256"),
        }
    }
}
