use hmac::{Hmac, Mac};
use sha2::Sha256;
use chrono::Utc;
use std::collections::HashMap;

/// Binance API 签名器
pub struct BinanceSignature {
    api_key: String,
    api_secret: String,
}

impl BinanceSignature {
    pub fn new(api_key: String, api_secret: String) -> Self {
        Self { api_key, api_secret }
    }

    /// 生成 Binance API 签名
    pub fn sign(&self, query_string: &str) -> String {
        let mut mac = Hmac::<Sha256>::new_from_slice(self.api_secret.as_bytes())
            .expect("HMAC can take keys of any size");
        mac.update(query_string.as_bytes());
        let result = mac.finalize();
        hex::encode(result.into_bytes())
    }

    /// 生成请求时间戳（毫秒）
    pub fn timestamp() -> u64 {
        Utc::now().timestamp_millis() as u64
    }

    /// 构建查询字符串
    pub fn build_query(params: &HashMap<String, String>) -> String {
        params
            .iter()
            .map(|(k, v)| format!("{}={}", k, v))
            .collect::<Vec<_>>()
            .join("&")
    }

    /// 构建查询字符串（从 slice）
    pub fn build_query_from_slice(params: &[(&str, &str)]) -> String {
        params
            .iter()
            .map(|(k, v)| format!("{}={}", k, v))
            .collect::<Vec<_>>()
            .join("&")
    }

    /// 获取 API Key
    pub fn api_key(&self) -> &str {
        &self.api_key
    }

    /// 获取请求头
    pub fn headers(&self) -> Vec<(&'static str, String)> {
        vec![
            ("X-MBX-APIKEY", self.api_key.clone()),
            ("Content-Type", "application/json".to_string()),
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_signature_generation() {
        let signature = BinanceSignature::new(
            "test_api_key".to_string(),
            "test_api_secret".to_string(),
        );

        let query_string = "symbol=BTCUSDT&side=BUY&type=LIMIT&quantity=1&price=50000&timestamp=1640000000000";
        let sig = signature.sign(query_string);

        // Signature should be a hex string
        assert_eq!(sig.len(), 64); // SHA256 = 64 hex chars
        assert!(sig.chars().all(|c| c.is_ascii_hexdigit()));
    }

    #[test]
    fn test_timestamp_generation() {
        let ts = BinanceSignature::timestamp();
        // Should be a reasonable timestamp (after 2020)
        assert!(ts > 1577836800000); // Jan 1, 2020
        assert!(ts < 2000000000000); // Reasonable upper bound
    }

    #[test]
    fn test_query_building() {
        let params = vec![
            ("symbol", "BTCUSDT"),
            ("side", "BUY"),
            ("type", "LIMIT"),
        ];

        let query = BinanceSignature::build_query_from_slice(&params);
        assert_eq!(query, "symbol=BTCUSDT&side=BUY&type=LIMIT");
    }

    #[test]
    fn test_api_key_access() {
        let signature = BinanceSignature::new(
            "my_key".to_string(),
            "my_secret".to_string(),
        );

        assert_eq!(signature.api_key(), "my_key");
    }
}
