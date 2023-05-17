use magic_crypt::{new_magic_crypt, MagicCrypt64, MagicCrypt128, MagicCrypt192, MagicCrypt256, MagicCryptTrait, MagicCryptError};

use super::enums::Standard;

#[derive(Debug, Clone)]
pub struct Cypher {
    key64: MagicCrypt64,
    key128: MagicCrypt128,
    key192: MagicCrypt192,
    key256: MagicCrypt256,
}

impl Cypher {
    pub fn new(key: &str) -> Self {
        Cypher {
            key64: new_magic_crypt!(key, 64),
            key128: new_magic_crypt!(key, 128),
            key192: new_magic_crypt!(key, 192),
            key256: new_magic_crypt!(key, 256),
        }
    }

    pub fn encrypt(&self, data: &str, standard: &Standard) -> Result<String, MagicCryptError> {
        let encrypted_str: String;
        match standard {
            Standard::DES64 => {
                encrypted_str = self.key64.encrypt_str_to_base64(data.to_string());
            },
            Standard::AES128 => {
                encrypted_str = self.key128.encrypt_str_to_base64(data.to_string());
            },
            Standard::AES192 => {
                encrypted_str = self.key192.encrypt_str_to_base64(data.to_string());
            },
            Standard::AES256 => {
                encrypted_str = self.key256.encrypt_str_to_base64(data.to_string());
            },
        }
        Ok(encrypted_str)
    }

    #[allow(dead_code)]
    pub fn decrypt(&self, data: &str, standard: &Standard) -> Result<String, MagicCryptError> {
        let decrypted_str: String;
        match standard {
            Standard::DES64 => {
                decrypted_str = self.key64.decrypt_base64_to_string(data.to_string())?;
            },
            Standard::AES128 => {
                decrypted_str = self.key128.decrypt_base64_to_string(data.to_string())?;
            },
            Standard::AES192 => {
                decrypted_str = self.key192.decrypt_base64_to_string(data.to_string())?;
            },
            Standard::AES256 => {
                decrypted_str = self.key256.decrypt_base64_to_string(data.to_string())?;
            },
        }
        Ok(decrypted_str)
    }
    
}

#[cfg(test)]
#[path = "./tests/crypto_test.rs"]
mod crypto_test;