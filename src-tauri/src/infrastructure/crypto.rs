use anyhow::Result;
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};

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
}
