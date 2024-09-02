//! API glue for implementing email sending. We send email using the `lettre`
//! crate which supports pluggable backends, so we can reconfigure which provider
//! to send emails with whenever needed.
//!
//! We don't host our own email sending because that's really difficult to
//! maintain.

use lettre::{
    transport::smtp::{
        authentication::Credentials,
        client::{Tls, TlsParameters},
    },
    AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor,
};
use serde::Deserialize;
use std::{sync::Arc, time::Duration};

#[derive(Deserialize, Debug)]
pub struct MailerConfiguration {
    pub smtp_host: String,
    pub smtp_port: u16,
    pub smtp_username: String,
    pub smtp_password: String,
}

impl MailerConfiguration {
    pub fn from_env() -> Self {
        envy::from_env().unwrap()
    }
}

#[derive(Clone, Debug)]
pub struct Mailer {
    transport: Arc<AsyncSmtpTransport<Tokio1Executor>>,
}

impl Mailer {
    pub fn new() -> Result<Self, lettre::transport::smtp::Error> {
        Self::from_config(MailerConfiguration::from_env())
    }

    pub fn from_config(
        config: MailerConfiguration,
    ) -> Result<Self, lettre::transport::smtp::Error> {
        let credentials = Credentials::new(config.smtp_username, config.smtp_password);

        let transport = AsyncSmtpTransport::<Tokio1Executor>::relay(&config.smtp_host)?
            .port(config.smtp_port)
            .tls(Tls::Required(TlsParameters::new(config.smtp_host)?))
            .timeout(Some(Duration::from_secs(5)))
            .credentials(credentials)
            .build();

        Ok(Self {
            transport: Arc::new(transport),
        })
    }

    pub async fn send(&self, message: Message) -> Result<(), lettre::transport::smtp::Error> {
        tracing::info!(
            recipients = ?message.envelope().to(),
            "sending an email",
        );

        self.transport.send(message).await?;

        Ok(())
    }
}
