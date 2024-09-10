//! API glue for implementing email sending. We send email using the `lettre`
//! crate which supports pluggable backends, so we can reconfigure which provider
//! to send emails with whenever needed.
//!
//! We don't host our own email sending because that's really difficult to
//! maintain.

use color_eyre::eyre::Result;
use lettre::{
    transport::{
        smtp::{
            authentication::Credentials,
            client::{Tls, TlsParameters},
        },
        stub::AsyncStubTransport,
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
    pub fn from_env() -> envy::Result<Self> {
        envy::from_env()
    }
}

/// Sends emails triggered by events in the website.
#[derive(Clone, Debug)]
pub struct Mailer(Repr);

#[derive(Clone, Debug)]
enum Repr {
    Smtp {
        transport: Arc<AsyncSmtpTransport<Tokio1Executor>>,
    },
    NoOp {
        transport: AsyncStubTransport,
    },
}

impl Mailer {
    /// Create a new mailer from the environment, selecting an implementation
    /// automatically.
    pub fn new() -> Result<Self> {
        match MailerConfiguration::from_env() {
            Ok(config) => Self::from_config(config),
            Err(error) => {
                tracing::warn!(?error, "no email configuration found in the environment, falling back to no-op: mailer");
                Ok(Self::no_op())
            }
        }
    }

    /// Create a new mailer that doesn't send any emails.
    pub fn no_op() -> Self {
        Self(Repr::NoOp {
            transport: AsyncStubTransport::new(Ok(())),
        })
    }

    pub fn from_config(config: MailerConfiguration) -> Result<Self> {
        let credentials = Credentials::new(config.smtp_username, config.smtp_password);

        let transport = AsyncSmtpTransport::<Tokio1Executor>::relay(&config.smtp_host)?
            .port(config.smtp_port)
            .tls(Tls::Required(TlsParameters::new(config.smtp_host)?))
            .timeout(Some(Duration::from_secs(5)))
            .credentials(credentials)
            .build();

        Ok(Self(Repr::Smtp {
            transport: Arc::new(transport),
        }))
    }

    pub async fn send(&self, message: Message) -> Result<()> {
        tracing::info!(
            recipients = ?message.envelope().to(),
            "sending an email",
        );

        match &self.0 {
            Repr::Smtp { transport } => {
                transport.send(message).await?;
            }
            Repr::NoOp { transport } => {
                transport.send(message).await?;
            }
        }

        Ok(())
    }
}
