//! Notification services
//!
//! This module provides various notification implementations for sending alerts
//! through different channels such as DingTalk webhooks and email.

use async_trait::async_trait;

pub mod dingtalk;
pub mod email;

pub use dingtalk::DingTalkNotifier;
pub use email::EmailNotifier;

/// Notification service trait for sending alerts via various channels
#[async_trait]
pub trait NotificationService: Send + Sync {
    /// Send DingTalk notification
    async fn send_dingtalk(&self, message: &str) -> Result<(), String>;

    /// Send email notification
    async fn send_email(&self, subject: &str, body: &str) -> Result<(), String>;
}

/// Implement NotificationService for DingTalkNotifier
#[async_trait]
impl NotificationService for DingTalkNotifier {
    async fn send_dingtalk(&self, message: &str) -> Result<(), String> {
        self.send(message)
            .await
            .map_err(|e| e.to_string())
    }

    async fn send_email(&self, _subject: &str, _body: &str) -> Result<(), String> {
        Err("Email not supported by DingTalkNotifier".to_string())
    }
}

/// Implement NotificationService for EmailNotifier
#[async_trait]
impl NotificationService for EmailNotifier {
    async fn send_dingtalk(&self, message: &str) -> Result<(), String> {
        // Fallback to email for DingTalk requests
        self.send("DingTalk Fallback", message)
            .await
            .map_err(|e| e.to_string())
    }

    async fn send_email(&self, subject: &str, body: &str) -> Result<(), String> {
        self.send(subject, body)
            .await
            .map_err(|e| e.to_string())
    }
}

/// Default notification service that logs messages
pub struct DefaultNotificationService;

#[async_trait]
impl NotificationService for DefaultNotificationService {
    async fn send_dingtalk(&self, message: &str) -> Result<(), String> {
        log::info!("[DingTalk Notification]\n{}", message);
        Ok(())
    }

    async fn send_email(&self, subject: &str, body: &str) -> Result<(), String> {
        log::info!("[Email Notification] Subject: {}\n{}", subject, body);
        Ok(())
    }
}
