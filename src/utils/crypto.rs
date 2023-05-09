use magic_crypt::{new_magic_crypt, MagicCrypt256, MagicCryptTrait, MagicCryptError};

#[derive(Debug, Clone)]
pub struct CryptoData {
    key: MagicCrypt256,
}

impl CryptoData {
    pub fn new(key: &str) -> Self {
        CryptoData {
            key: new_magic_crypt!(key, 256),
        }
    }
    pub fn encrypt(&self, data: &str) -> Result<String, MagicCryptError> {
        Ok(self.key.encrypt_str_to_base64(data.to_string()))
    }

    #[allow(dead_code)]
    pub fn decrypt(&self, data: &str) -> Result<String, MagicCryptError> {
        Ok(self.key.decrypt_base64_to_string(data)?)
    }
}

#[cfg(test)]
#[path = "./tests/crypto_test.rs"]
mod crypto_test;