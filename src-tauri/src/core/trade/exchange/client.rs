use reqwest::Client;
use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE};
use serde_json::Value;
use super::signature::BinanceSignature;

/// Binance REST API 客户端
pub struct BinanceClient {
    client: Client,
    api_key: String,
    api_secret: String,
    base_url: String,
}

impl BinanceClient {
    /// 创建新的客户端实例
    pub fn new(api_key: String, api_secret: String, testnet: bool) -> Self {
        let base_url = if testnet {
            "https://testnet.binance.vision"
        } else {
            "https://api.binance.com"
        }.to_string();

        Self {
            client: Client::new(),
            api_key,
            api_secret,
            base_url,
        }
    }

    /// 构建请求头
    fn build_headers(&self) -> HeaderMap {
        let mut headers = HeaderMap::new();
        if let Ok(api_key_val) = HeaderValue::from_str(&self.api_key) {
            headers.insert("X-MBX-APIKEY", api_key_val);
        }
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        headers
    }

    /// 发送 GET 请求（公开端点）
    pub async fn get(&self, endpoint: &str, params: &[(&str, &str)]) -> Result<Value, String> {
        let query = if params.is_empty() {
            String::new()
        } else {
            format!("?{}", BinanceSignature::build_query_from_slice(params))
        };
        let url = format!("{}{}{}", self.base_url, endpoint, query);

        self.client
            .get(&url)
            .send()
            .await
            .map_err(|e| format!("GET request failed: {}", e))?
            .json()
            .await
            .map_err(|e| format!("Response parse failed: {}", e))
    }

    /// 发送 GET 请求（需要签名）
    pub async fn get_signed(&self, endpoint: &str, params: &[(&str, &str)]) -> Result<Value, String> {
        let signature = BinanceSignature::new(self.api_key.clone(), self.api_secret.clone());
        let timestamp = BinanceSignature::timestamp();
        let timestamp_str = timestamp.to_string();

        // 构建查询字符串（包含时间戳）
        let mut all_params = params.to_vec();
        all_params.push(("timestamp", timestamp_str.as_str()));

        let query = BinanceSignature::build_query_from_slice(&all_params);
        let sig = signature.sign(&query);

        let url = format!(
            "{}{}?{}&signature={}",
            self.base_url, endpoint, query, sig
        );

        let response = self.client
            .get(&url)
            .headers(self.build_headers())
            .send()
            .await
            .map_err(|e| format!("Signed GET request failed: {}", e))?;

        self.handle_response(response).await
    }

    /// 发送 POST 请求（需要签名）
    pub async fn post_signed(&self, endpoint: &str, params: &[(&str, &str)]) -> Result<Value, String> {
        let signature = BinanceSignature::new(self.api_key.clone(), self.api_secret.clone());
        let timestamp = BinanceSignature::timestamp();
        let timestamp_str = timestamp.to_string();

        // 构建查询字符串（包含时间戳）
        let mut all_params = params.to_vec();
        all_params.push(("timestamp", timestamp_str.as_str()));

        let query = BinanceSignature::build_query_from_slice(&all_params);
        let sig = signature.sign(&query);

        let url = format!(
            "{}{}?{}&signature={}",
            self.base_url, endpoint, query, sig
        );

        let response = self.client
            .post(&url)
            .headers(self.build_headers())
            .send()
            .await
            .map_err(|e| format!("Signed POST request failed: {}", e))?;

        self.handle_response(response).await
    }

    /// 发送 DELETE 请求（需要签名）
    pub async fn delete_signed(&self, endpoint: &str, params: &[(&str, &str)]) -> Result<Value, String> {
        let signature = BinanceSignature::new(self.api_key.clone(), self.api_secret.clone());
        let timestamp = BinanceSignature::timestamp();
        let timestamp_str = timestamp.to_string();

        // 构建查询字符串（包含时间戳）
        let mut all_params = params.to_vec();
        all_params.push(("timestamp", timestamp_str.as_str()));

        let query = BinanceSignature::build_query_from_slice(&all_params);
        let sig = signature.sign(&query);

        let url = format!(
            "{}{}?{}&signature={}",
            self.base_url, endpoint, query, sig
        );

        let response = self.client
            .delete(&url)
            .headers(self.build_headers())
            .send()
            .await
            .map_err(|e| format!("Signed DELETE request failed: {}", e))?;

        self.handle_response(response).await
    }

    /// 发送 PUT 请求（需要签名）
    pub async fn put_signed(&self, endpoint: &str, params: &[(&str, &str)]) -> Result<Value, String> {
        let signature = BinanceSignature::new(self.api_key.clone(), self.api_secret.clone());
        let timestamp = BinanceSignature::timestamp();
        let timestamp_str = timestamp.to_string();

        // 构建查询字符串（包含时间戳）
        let mut all_params = params.to_vec();
        all_params.push(("timestamp", timestamp_str.as_str()));

        let query = BinanceSignature::build_query_from_slice(&all_params);
        let sig = signature.sign(&query);

        let url = format!(
            "{}{}?{}&signature={}",
            self.base_url, endpoint, query, sig
        );

        let response = self.client
            .put(&url)
            .headers(self.build_headers())
            .send()
            .await
            .map_err(|e| format!("Signed PUT request failed: {}", e))?;

        self.handle_response(response).await
    }

    /// 处理 API 响应，检查错误代码
    async fn handle_response(&self, response: reqwest::Response) -> Result<Value, String> {
        let status = response.status();

        if status.is_success() {
            response
                .json()
                .await
                .map_err(|e| format!("Response parse failed: {}", e))
        } else {
            // 尝试解析错误响应
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());

            if let Ok(error_json) = serde_json::from_str::<Value>(&error_text) {
                if let Some(msg) = error_json.get("msg").and_then(|m| m.as_str()) {
                    if let Some(code) = error_json.get("code").and_then(|c| c.as_i64()) {
                        return Err(format!("Binance API error {}: {}", code, msg));
                    }
                    return Err(format!("Binance API error: {}", msg));
                }
            }

            Err(format!("HTTP error {}: {}", status.as_u16(), error_text))
        }
    }

    /// 获取基础 URL
    pub fn base_url(&self) -> &str {
        &self.base_url
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_creation() {
        let client = BinanceClient::new(
            "test_key".to_string(),
            "test_secret".to_string(),
            true,
        );

        assert_eq!(client.base_url, "https://testnet.binance.vision");
    }

    #[test]
    fn test_client_production_url() {
        let client = BinanceClient::new(
            "test_key".to_string(),
            "test_secret".to_string(),
            false,
        );

        assert_eq!(client.base_url, "https://api.binance.com");
    }
}
