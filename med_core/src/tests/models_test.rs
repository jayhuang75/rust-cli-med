use clap::{builder::PossibleValue, ValueEnum};

use crate::models::{
    enums::{AppMode, FileType, Mode, Standard},
    params::Params,
};

#[tokio::test]
async fn test_enum() {
    let mode = Mode::MASK;
    assert_eq!(mode.to_string(), "mask");
    assert_eq!(format!("{mode:?}"), "MASK");

    let mode = Mode::ENCRYPT;
    assert_eq!(mode.to_string(), "encrypt");
    assert_eq!(format!("{mode:?}"), "ENCRYPT");

    let mode = Mode::DECRYPT;
    assert_eq!(mode.to_string(), "decrypt");
    assert_eq!(format!("{mode:?}"), "DECRYPT");

    let app_mode = AppMode::CLI;
    assert_eq!(app_mode.to_string(), "CLI");
    assert_eq!(format!("{app_mode:?}"), "CLI");

    let app_mode = AppMode::SDK;
    assert_eq!(app_mode.to_string(), "SDK");
    assert_eq!(format!("{app_mode:?}"), "SDK");

    let new_standard = Standard::DES64;
    assert_eq!(new_standard.to_string(), "des64");
    assert_eq!(format!("{new_standard:?}"), "DES64");

    let new_standard = Standard::AES128;
    assert_eq!(new_standard.to_string(), "aes128");
    assert_eq!(format!("{new_standard:?}"), "AES128");

    let new_standard = Standard::AES192;
    assert_eq!(new_standard.to_string(), "aes192");
    assert_eq!(format!("{new_standard:?}"), "AES192");

    let new_standard = Standard::AES256;
    assert_eq!(new_standard.to_string(), "aes256");
    assert_eq!(format!("{new_standard:?}"), "AES256");

    assert_eq!(
        Standard::value_variants(),
        &[
            Standard::DES64,
            Standard::AES128,
            Standard::AES192,
            Standard::AES256
        ]
    );
    assert_eq!(
        Standard::to_possible_value(&Standard::DES64),
        Some(PossibleValue::new("des64").help("DES standard 64"))
    );
    assert_eq!(
        Standard::to_possible_value(&Standard::AES128),
        Some(PossibleValue::new("aes128").help("AES standard 128"))
    );
    assert_eq!(
        Standard::to_possible_value(&Standard::AES192),
        Some(PossibleValue::new("aes192").help("AES standard 192"))
    );
    assert_eq!(
        Standard::to_possible_value(&Standard::AES256),
        Some(PossibleValue::new("aes256").help("AES standard 256"))
    );

    match Standard::from_str("des64", true) {
        Ok(s) => {
            assert_eq!(s, Standard::DES64);
        }
        Err(_) => {
            unimplemented!()
        }
    }

    match Standard::from_str("test", true) {
        Ok(_) => {
            unimplemented!()
        }
        Err(e) => {
            assert_eq!(e, "invalid variant: test");
        }
    }

    assert_eq!(
        Mode::value_variants(),
        &[Mode::MASK, Mode::ENCRYPT, Mode::DECRYPT]
    );
    assert_eq!(
        Mode::to_possible_value(&Mode::MASK),
        Some(PossibleValue::new("mask").help("Masking the data"))
    );
    assert_eq!(
        Mode::to_possible_value(&Mode::ENCRYPT),
        Some(PossibleValue::new("encrypt").help("Encrypt the data with provided KEY"))
    );
    assert_eq!(
        Mode::to_possible_value(&Mode::DECRYPT),
        Some(PossibleValue::new("decrypt").help("Decrypt the data with provided KEY"))
    );

    match Mode::from_str("mask", true) {
        Ok(s) => {
            assert_eq!(s, Mode::MASK);
        }
        Err(_) => {
            unimplemented!()
        }
    }

    match Mode::from_str("test", true) {
        Ok(_) => {
            unimplemented!()
        }
        Err(e) => {
            assert_eq!(e, "invalid variant: test");
        }
    }
}

#[tokio::test]
async fn test_params_init() {
    let new_params = Params::default();
    assert_eq!(new_params.app_mode, AppMode::CLI);
    assert_eq!(new_params.debug, false);
    assert_eq!(new_params.file_type, FileType::CSV);
    assert_eq!(new_params.file_path, "");
    assert_eq!(new_params.conf_path, "");
    assert_eq!(new_params.mode, Mode::MASK);
    assert_eq!(new_params.standard, Standard::DES64);
    assert_eq!(new_params.key, Some("".to_owned()));
    assert_eq!(new_params.to_string(), "app_mode: CLI, file_path: , file_type: csv, conf_path: , output_path: , mode: mask, key: Some(\"\"), debug: false, worker: 2");
}
