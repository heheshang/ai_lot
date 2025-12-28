//! DingTalk webhook notification service
//!
//! This module implements a notification service that sends alerts to DingTalk
//! using webhook URLs.

use anyhow::{Context, Result};
use reqwest::Client;
use serde::{Deserialize, Serialize};

/// DingTalk webhook notification sender
///
/// This struct handles sending notifications to DingTalk via webhook URLs.
/// It maintains an HTTP client for efficient connection reuse.
#[derive(Clone, Debug)]
pub struct DingTalkNotifier {
    /// DingTalk webhook URL
    webhook_url: String,
    /// HTTP client for sending requests
    client: Client,
}

/// DingTalk message wrapper
///
/// This represents the top-level structure of a DingTalk webhook message.
#[derive(Serialize, Debug)]
struct DingTalkMessage {
    /// Message type (always "text" for basic messages)
    msgtype: String,
    /// Text content of the message
    text: DingTalkText,
}

/// DingTalk text message content
///
/// Contains the actual message text to be sent.
#[derive(Serialize, Debug)]
struct DingTalkText {
    /// Message content
    content: String,
}

/// DingTalk API response
///
/// Represents the response from DingTalk webhook API.
#[derive(Deserialize, Debug)]
struct DingTalkResponse {
    /// Response error code (0 for success)
    errcode: i32,
    /// Error message
    errmsg: String,
}

impl DingTalkNotifier {
    /// Create a new DingTalk notifier
    ///
    /// # Arguments
    /// * `webhook_url` - DingTalk webhook URL
    ///
    /// # Example
    /// ```no_run
    /// use ai_lot_lib::infrastructure::notification::DingTalkNotifier;
    ///
    /// let notifier = DingTalkNotifier::new(
    ///     "https://oapi.dingtalk.com/robot/send?access_token=xxx".to_string()
    /// );
    /// ```
    pub fn new(webhook_url: String) -> Self {
        Self {
            webhook_url,
            client: Client::new(),
        }
    }

    /// Create a new DingTalk notifier with custom HTTP client
    ///
    /// This allows configuring the HTTP client with custom settings
    /// such as timeouts or proxies.
    ///
    /// # Arguments
    /// * `webhook_url` - DingTalk webhook URL
    /// * `client` - Custom reqwest client
    pub fn with_client(webhook_url: String, client: Client) -> Self {
        Self {
            webhook_url,
            client,
        }
    }

    /// Send a notification message
    ///
    /// # Arguments
    /// * `message` - Message text to send
    ///
    /// # Returns
    /// Returns `Ok(())` if the message was sent successfully, or an error otherwise.
    ///
    /// # Errors
    /// Returns an error if:
    /// - The HTTP request fails
    /// - The response indicates a failure (non-success status code)
    /// - The DingTalk API returns an error code
    ///
    /// # Example
    /// ```no_run
    /// # use ai_lot_lib::infrastructure::notification::DingTalkNotifier;
    /// # async fn example() -> anyhow::Result<()> {
    /// # let notifier = DingTalkNotifier::new("https://oapi.dingtalk.com/robot/send?access_token=xxx".to_string());
    /// notifier.send("Risk alert triggered!").await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn send(&self, message: &str) -> Result<()> {
        // Build the message payload with AI-LOT prefix
        let payload = DingTalkMessage {
            msgtype: "text".to_string(),
            text: DingTalkText {
                content: format!("[AI-LOT 风控预警]\n{}", message),
            },
        };

        // Send POST request to DingTalk webhook
        let response = self
            .client
            .post(&self.webhook_url)
            .json(&payload)
            .send()
            .await
            .context("Failed to send DingTalk webhook request")?;

        // Check HTTP status code
        if !response.status().is_success() {
            let status = response.status();
            let error_body = response.text().await.unwrap_or_else(|_| "Unable to read error body".to_string());
            anyhow::bail!(
                "DingTalk API returned error status {}: {}",
                status,
                error_body
            );
        }

        // Parse and check API response
        let api_response: DingTalkResponse = response
            .json()
            .await
            .context("Failed to parse DingTalk API response")?;

        if api_response.errcode != 0 {
            anyhow::bail!(
                "DingTalk API returned error code {}: {}",
                api_response.errcode,
                api_response.errmsg
            );
        }

        log::info!("DingTalk notification sent successfully");
        Ok(())
    }

    /// Get the webhook URL
    pub fn webhook_url(&self) -> &str {
        &self.webhook_url
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Note: Comprehensive integration tests would require:
    // 1. Mock HTTP server (e.g., mockito or wiremock)
    // 2. Actual DingTalk webhook URL for manual testing
    //
    // For now, we provide basic compilation tests

    #[test]
    fn test_dingtalk_notifier_creation() {
        let webhook_url = "https://oapi.dingtalk.com/robot/send?access_token=test";
        let notifier = DingTalkNotifier::new(webhook_url.to_string());

        assert_eq!(notifier.webhook_url(), webhook_url);
    }

    #[test]
    fn test_dingtalk_notifier_with_custom_client() {
        let client = Client::builder()
            .timeout(std::time::Duration::from_secs(30))
            .build()
            .unwrap();

        let webhook_url = "https://oapi.dingtalk.com/robot/send?access_token=test";
        let notifier = DingTalkNotifier::with_client(webhook_url.to_string(), client);

        assert_eq!(notifier.webhook_url(), webhook_url);
    }

    #[test]
    fn test_dingtalk_message_serialization() {
        let message = DingTalkMessage {
            msgtype: "text".to_string(),
            text: DingTalkText {
                content: "Test message".to_string(),
            },
        };

        let json = serde_json::to_string(&message).unwrap();
        assert!(json.contains("text"));
        assert!(json.contains("Test message"));
    }

    #[test]
    fn test_dingtalk_response_deserialization() {
        let json = r#"{"errcode": 0, "errmsg": "ok"}"#;
        let response: DingTalkResponse = serde_json::from_str(json).unwrap();

        assert_eq!(response.errcode, 0);
        assert_eq!(response.errmsg, "ok");
    }

    #[test]
    fn test_dingtalk_response_error() {
        let json = r#"{"errcode": 300001, "errmsg": "access_token invalid"}"#;
        let response: DingTalkResponse = serde_json::from_str(json).unwrap();

        assert_eq!(response.errcode, 300001);
        assert!(response.errmsg.contains("invalid"));
    }

    #[test]
    fn test_message_formatting() {
        let webhook_url = "https://oapi.dingtalk.com/robot/send?access_token=test";
        let notifier = DingTalkNotifier::new(webhook_url.to_string());

        // Verify webhook URL is stored correctly
        assert!(notifier.webhook_url().contains("dingtalk.com"));
        assert!(notifier.webhook_url().contains("access_token"));
    }

    // Manual integration test (disabled by default)
    // Uncomment and set DINGTALK_WEBHOOK_URL environment variable to run
    #[tokio::test]
    #[ignore]
    async fn test_send_real_notification() {
        // This test requires a real DingTalk webhook URL
        // Run with: DINGTALK_WEBHOOK_URL=https://... cargo test test_send_real_notification -- --ignored

        let webhook_url = std::env::var("DINGTALK_WEBHOOK_URL")
            .expect("Set DINGTALK_WEBHOOK_URL environment variable to run this test");

        let notifier = DingTalkNotifier::new(webhook_url);
        let result = notifier
            .send("Test notification from AI-LOT (this is a test)")
            .await;

        assert!(
            result.is_ok(),
            "Failed to send notification: {:?}",
            result.err()
        );
    }
}
