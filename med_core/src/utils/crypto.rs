use magic_crypt::{
    new_magic_crypt, MagicCrypt128, MagicCrypt192, MagicCrypt256, MagicCrypt64, MagicCryptError,
    MagicCryptTrait,
};

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
        let encrypted_str: String = match standard {
            Standard::DES64 => self.key64.encrypt_str_to_base64(data),
            Standard::AES128 => self.key128.encrypt_str_to_base64(data),
            Standard::AES192 => self.key192.encrypt_str_to_base64(data),
            Standard::AES256 => self.key256.encrypt_str_to_base64(data),
        };
        Ok(encrypted_str)
    }

    #[allow(dead_code)]
    pub fn decrypt(&self, data: &str, standard: &Standard) -> Result<String, MagicCryptError> {
        let decrypted_str: String = match standard {
            Standard::DES64 => self.key64.decrypt_base64_to_string(data)?,
            Standard::AES128 => self.key128.decrypt_base64_to_string(data)?,
            Standard::AES192 => self.key192.decrypt_base64_to_string(data)?,
            Standard::AES256 => self.key256.decrypt_base64_to_string(data)?,
        };
        Ok(decrypted_str)
    }
}

#[cfg(test)]
#[path = "./tests/crypto_test.rs"]
mod crypto_test;
