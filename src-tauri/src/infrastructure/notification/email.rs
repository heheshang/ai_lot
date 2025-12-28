use anyhow::{Context, Result};
use lettre::{
    message::header::ContentType,
    transport::smtp::authentication::Credentials,
    Message, SmtpTransport, Transport,
};

/// Email notification service using lettre
#[derive(Clone, Debug)]
pub struct EmailNotifier {
    smtp_server: String,
    smtp_username: String,
    smtp_password: String,
    from_email: String,
    to_emails: Vec<String>,
}

impl EmailNotifier {
    /// Create a new email notifier
    ///
    /// # Arguments
    /// * `smtp_server` - SMTP server address (e.g., "smtp.gmail.com")
    /// * `smtp_username` - SMTP username/email
    /// * `smtp_password` - SMTP password or app password
    /// * `from_email` - Sender email address
    /// * `to_emails` - List of recipient email addresses
    pub fn new(
        smtp_server: String,
        smtp_username: String,
        smtp_password: String,
        from_email: String,
        to_emails: Vec<String>,
    ) -> Self {
        Self {
            smtp_server,
            smtp_username,
            smtp_password,
            from_email,
            to_emails,
        }
    }

    /// Send email notification
    ///
    /// # Arguments
    /// * `subject` - Email subject line
    /// * `body` - Email body content
    ///
    /// # Returns
    /// Returns Ok(()) if all emails sent successfully
    /// Returns error if any email fails to send
    pub async fn send(&self, subject: &str, body: &str) -> Result<()> {
        let credentials = Credentials::new(self.smtp_username.clone(), self.smtp_password.clone());

        // Create SMTP transport with relay
        let mailer = SmtpTransport::relay(&self.smtp_server)
            .context(format!("Failed to create SMTP transport for server: {}", self.smtp_server))?
            .credentials(credentials)
            .build();

        // Send email to each recipient
        for to_email in &self.to_emails {
            let email = Message::builder()
                .from(self.from_email.parse()?)
                .to(to_email.parse()?)
                .subject(subject)
                .header(ContentType::TEXT_PLAIN)
                .body(body.to_string())
                .with_context(|| format!("Failed to build email for recipient: {}", to_email))?;

            mailer
                .send(&email)
                .with_context(|| format!("Failed to send email to: {}", to_email))?;

            log::info!("Email sent successfully to: {}", to_email);
        }

        Ok(())
    }

    /// Send HTML email notification
    ///
    /// # Arguments
    /// * `subject` - Email subject line
    /// * `html_body` - HTML email body content
    ///
    /// # Returns
    /// Returns Ok(()) if all emails sent successfully
    pub async fn send_html(&self, subject: &str, html_body: &str) -> Result<()> {
        let credentials = Credentials::new(self.smtp_username.clone(), self.smtp_password.clone());

        let mailer = SmtpTransport::relay(&self.smtp_server)?
            .credentials(credentials)
            .build();

        for to_email in &self.to_emails {
            let email = Message::builder()
                .from(self.from_email.parse()?)
                .to(to_email.parse()?)
                .subject(subject)
                .header(ContentType::TEXT_HTML)
                .body(html_body.to_string())
                .with_context(|| format!("Failed to build HTML email for: {}", to_email))?;

            mailer
                .send(&email)
                .with_context(|| format!("Failed to send HTML email to: {}", to_email))?;

            log::info!("HTML email sent successfully to: {}", to_email);
        }

        Ok(())
    }

    /// Get recipient count
    pub fn recipient_count(&self) -> usize {
        self.to_emails.len()
    }

    /// Add a recipient
    pub fn add_recipient(&mut self, email: String) {
        self.to_emails.push(email);
    }

    /// Remove a recipient
    pub fn remove_recipient(&mut self, email: &str) {
        self.to_emails.retain(|e| e != email);
    }

    /// Clear all recipients
    pub fn clear_recipients(&mut self) {
        self.to_emails.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_email_notifier_creation() {
        let notifier = EmailNotifier::new(
            "smtp.test.com".to_string(),
            "user@test.com".to_string(),
            "password".to_string(),
            "from@test.com".to_string(),
            vec!["to1@test.com".to_string(), "to2@test.com".to_string()],
        );

        assert_eq!(notifier.recipient_count(), 2);
    }

    #[test]
    fn test_add_recipient() {
        let mut notifier = EmailNotifier::new(
            "smtp.test.com".to_string(),
            "user@test.com".to_string(),
            "password".to_string(),
            "from@test.com".to_string(),
            vec!["to1@test.com".to_string()],
        );

        notifier.add_recipient("to2@test.com".to_string());
        assert_eq!(notifier.recipient_count(), 2);
    }

    #[test]
    fn test_remove_recipient() {
        let mut notifier = EmailNotifier::new(
            "smtp.test.com".to_string(),
            "user@test.com".to_string(),
            "password".to_string(),
            "from@test.com".to_string(),
            vec!["to1@test.com".to_string(), "to2@test.com".to_string()],
        );

        notifier.remove_recipient("to1@test.com");
        assert_eq!(notifier.recipient_count(), 1);
        assert_eq!(notifier.to_emails[0], "to2@test.com");
    }

    #[test]
    fn test_clear_recipients() {
        let mut notifier = EmailNotifier::new(
            "smtp.test.com".to_string(),
            "user@test.com".to_string(),
            "password".to_string(),
            "from@test.com".to_string(),
            vec!["to1@test.com".to_string(), "to2@test.com".to_string()],
        );

        notifier.clear_recipients();
        assert_eq!(notifier.recipient_count(), 0);
    }
}
