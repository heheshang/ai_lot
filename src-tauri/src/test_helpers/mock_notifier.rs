//! Mock notification service for testing
//!
//! This module provides a mock implementation of the NotificationService trait
//! that records notifications instead of actually sending them.

use crate::infrastructure::NotificationService;
use async_trait::async_trait;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Mock notification service that tracks calls
#[derive(Clone)]
pub struct MockNotifier {
    /// Record of DingTalk messages sent
    dingtalk_messages: Arc<RwLock<Vec<String>>>,
    /// Record of email subjects sent
    email_subjects: Arc<RwLock<Vec<String>>>,
    /// Record of email bodies sent
    email_bodies: Arc<RwLock<Vec<String>>>,
}

impl MockNotifier {
    /// Create a new mock notifier
    pub fn new() -> Self {
        Self {
            dingtalk_messages: Arc::new(RwLock::new(Vec::new())),
            email_subjects: Arc::new(RwLock::new(Vec::new())),
            email_bodies: Arc::new(RwLock::new(Vec::new())),
        }
    }

    /// Get all DingTalk messages sent
    pub async fn get_dingtalk_messages(&self) -> Vec<String> {
        self.dingtalk_messages.read().await.clone()
    }

    /// Get all email subjects sent
    pub async fn get_email_subjects(&self) -> Vec<String> {
        self.email_subjects.read().await.clone()
    }

    /// Get all email bodies sent
    pub async fn get_email_bodies(&self) -> Vec<String> {
        self.email_bodies.read().await.clone()
    }

    /// Clear all recorded messages
    pub async fn clear(&self) {
        self.dingtalk_messages.write().await.clear();
        self.email_subjects.write().await.clear();
        self.email_bodies.write().await.clear();
    }

    /// Get count of DingTalk messages sent
    pub async fn dingtalk_count(&self) -> usize {
        self.dingtalk_messages.read().await.len()
    }

    /// Get count of emails sent
    pub async fn email_count(&self) -> usize {
        self.email_subjects.read().await.len()
    }

    /// Check if a specific message was sent via DingTalk
    pub async fn contains_dingtalk(&self, text: &str) -> bool {
        self.dingtalk_messages
            .read()
            .await
            .iter()
            .any(|msg| msg.contains(text))
    }

    /// Check if a specific subject was sent via email
    pub async fn contains_email_subject(&self, text: &str) -> bool {
        self.email_subjects
            .read()
            .await
            .iter()
            .any(|subject| subject.contains(text))
    }

    /// Check if a specific text was sent in email body
    pub async fn contains_email_body(&self, text: &str) -> bool {
        self.email_bodies
            .read()
            .await
            .iter()
            .any(|body| body.contains(text))
    }
}

impl Default for MockNotifier {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl NotificationService for MockNotifier {
    async fn send_dingtalk(&self, message: &str) -> Result<(), String> {
        self.dingtalk_messages
            .write()
            .await
            .push(message.to_string());
        log::info!("[Mock DingTalk] {}", message);
        Ok(())
    }

    async fn send_email(&self, subject: &str, body: &str) -> Result<(), String> {
        self.email_subjects.write().await.push(subject.to_string());
        self.email_bodies.write().await.push(body.to_string());
        log::info!("[Mock Email] Subject: {}", subject);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_mock_notifier_records_dingtalk() {
        let notifier = MockNotifier::new();

        notifier
            .send_dingtalk("Test message")
            .await
            .expect("Failed to send");

        assert_eq!(notifier.dingtalk_count().await, 1);
        assert!(notifier.contains_dingtalk("Test message").await);
    }

    #[tokio::test]
    async fn test_mock_notifier_records_email() {
        let notifier = MockNotifier::new();

        notifier
            .send_email("Test Subject", "Test Body")
            .await
            .expect("Failed to send");

        assert_eq!(notifier.email_count().await, 1);
        assert!(notifier.contains_email_subject("Test Subject").await);
        assert!(notifier.contains_email_body("Test Body").await);
    }

    #[tokio::test]
    async fn test_mock_notifier_clear() {
        let notifier = MockNotifier::new();

        notifier.send_dingtalk("Message 1").await.unwrap();
        notifier.send_dingtalk("Message 2").await.unwrap();
        notifier.send_email("Subject", "Body").await.unwrap();

        assert_eq!(notifier.dingtalk_count().await, 2);
        assert_eq!(notifier.email_count().await, 1);

        notifier.clear().await;

        assert_eq!(notifier.dingtalk_count().await, 0);
        assert_eq!(notifier.email_count().await, 0);
    }
}
