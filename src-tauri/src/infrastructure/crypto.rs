use anyhow::Result;
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use aes_gcm::{
    aead::{Aead, AeadCore, KeyInit},
    Aes256Gcm, Nonce,
};
use std::env;

/// 密码哈希服务
pub struct CryptoService;

impl CryptoService {
    /// 哈希密码
    ///
    /// 使用 Argon2id 算法哈希密码，返回密码哈希字符串
    pub fn hash_password(password: &str) -> Result<String> {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        let password_hash = argon2
            .hash_password(password.as_bytes(), &salt)
            .map_err(|e| anyhow::anyhow!("Password hash failed: {}", e))?;
        log::info!("password:{password},password_hash:{{password_hash:?}}",);
        Ok(password_hash.to_string())
    }

    /// 验证密码
    ///
    /// 验证明文密码与哈希值是否匹配
    pub fn verify_password(password: &str, hash: &str) -> Result<bool> {
        let parsed_hash = PasswordHash::new(hash)
            .map_err(|e| anyhow::anyhow!("Invalid password hash: {}", e))?;
        let argon2 = Argon2::default();
        match argon2.verify_password(password.as_bytes(), &parsed_hash) {
            Ok(()) => Ok(true),
            Err(_) => Ok(false),
        }
    }

    /// 生成随机盐值
    ///
    /// 生成用于密码哈希的随机盐值字符串
    pub fn generate_salt() -> String {
        SaltString::generate(&mut OsRng).to_string()
    }

    /// 获取加密密钥
    ///
    /// 从环境变量或派生密钥获取加密密钥
    fn get_encryption_key() -> Result<[u8; 32]> {
        // 优先从环境变量获取
        if let Ok(key_str) = env::var("AI_LOT_ENCRYPTION_KEY") {
            // 从十六进制字符串解析密钥
            if key_str.len() == 64 {
                let mut key = [0u8; 32];
                for i in 0..32 {
                    key[i] = u8::from_str_radix(&key_str[i * 2..i * 2 + 2], 16)
                        .map_err(|_| anyhow::anyhow!("Invalid encryption key format"))?;
                }
                return Ok(key);
            }
        }

        // 备用方案：使用机器特定密钥
        // 在生产环境中应该使用更安全的方式，如 HSM 或密钥管理服务
        const MACHINE_KEY: &[u8] = b"AI_LOT_DEFAULT_ENCRYPTION_KEY_2025_32";
        let mut key = [0u8; 32];
        key.copy_from_slice(&MACHINE_KEY[..32]);
        Ok(key)
    }

    /// 加密 API 密钥
    ///
    /// 使用 AES-256-GCM 加密敏感数据
    pub fn encrypt_api_key(data: &str) -> Result<String> {
        let key = Self::get_encryption_key()?;
        let cipher = Aes256Gcm::new(&key.into());
        let nonce = Aes256Gcm::generate_nonce(&mut OsRng);

        let ciphertext = cipher
            .encrypt(&nonce, data.as_bytes())
            .map_err(|e| anyhow::anyhow!("Encryption failed: {}", e))?;

        // 组合 nonce 和 ciphertext，使用 hex 编码
        Ok(format!(
            "{}:{}",
            hex::encode(nonce),
            hex::encode(ciphertext)
        ))
    }

    /// 解密 API 密钥
    ///
    /// 解密使用 encrypt_api_key 加密的数据
    pub fn decrypt_api_key(encrypted: &str) -> Result<String> {
        let key = Self::get_encryption_key()?;
        let cipher = Aes256Gcm::new(&key.into());

        let parts: Vec<&str> = encrypted.splitn(2, ':').collect();
        if parts.len() != 2 {
            return Err(anyhow::anyhow!("Invalid encrypted format"));
        }

        let nonce_bytes = hex::decode(parts[0])
            .map_err(|_| anyhow::anyhow!("Invalid nonce format"))?;
        let ciphertext = hex::decode(parts[1])
            .map_err(|_| anyhow::anyhow!("Invalid ciphertext format"))?;

        if nonce_bytes.len() != 12 {
            return Err(anyhow::anyhow!("Invalid nonce length"));
        }

        let nonce = Nonce::from_slice(&nonce_bytes);

        let plaintext = cipher
            .decrypt(nonce, ciphertext.as_slice())
            .map_err(|_| anyhow::anyhow!("Decryption failed"))?;

        String::from_utf8(plaintext)
            .map_err(|_| anyhow::anyhow!("Invalid UTF-8 in decrypted data"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_password() {
        let password = "test_password_123";
        let hash = CryptoService::hash_password(password).unwrap();
        assert!(hash.len() > 0);
        assert_ne!(hash, password);
    }

    #[test]
    fn test_verify_password() {
        let password = "test_password_123";
        let hash = CryptoService::hash_password(password).unwrap();
        assert!(CryptoService::verify_password(password, &hash).unwrap());
        assert!(!CryptoService::verify_password("wrong_password", &hash).unwrap());
    }

    #[test]
    fn test_generate_salt() {
        let salt1 = CryptoService::generate_salt();
        let salt2 = CryptoService::generate_salt();
        assert_ne!(salt1, salt2);
    }

    #[test]
    fn test_encrypt_decrypt_api_key() {
        let api_key = "my_secret_api_key_12345";
        let encrypted = CryptoService::encrypt_api_key(api_key).unwrap();

        // 加密后的数据应该与原数据不同
        assert_ne!(encrypted, api_key);
        assert!(encrypted.contains(':'));
        assert!(encrypted.len() > 20);

        // 解密应该得到原数据
        let decrypted = CryptoService::decrypt_api_key(&encrypted).unwrap();
        assert_eq!(decrypted, api_key);
    }

    #[test]
    fn test_decrypt_invalid_format() {
        let result = CryptoService::decrypt_api_key("invalid_format");
        assert!(result.is_err());
    }

    #[test]
    fn test_decrypt_invalid_hex() {
        let result = CryptoService::decrypt_api_key("invalid:hex");
        assert!(result.is_err());
    }

    #[test]
    fn test_encrypt_twice_different_results() {
        let api_key = "my_secret_api_key";
        let encrypted1 = CryptoService::encrypt_api_key(api_key).unwrap();
        let encrypted2 = CryptoService::encrypt_api_key(api_key).unwrap();

        // 每次加密应该产生不同的结果（因为 nonce 随机）
        assert_ne!(encrypted1, encrypted2);

        // 但解密后应该相同
        assert_eq!(CryptoService::decrypt_api_key(&encrypted1).unwrap(), api_key);
        assert_eq!(CryptoService::decrypt_api_key(&encrypted2).unwrap(), api_key);
    }
}
