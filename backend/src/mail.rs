use lettre::AsyncTransport;
use std::sync::Arc;

pub const MAIL_PASSWORD_RESET: &str = include_str!("../static/mails/password_reset.html");
pub const MAIL_ACCOUNT_CREATED: &str = include_str!("../static/mails/account_created.html");

#[derive(Debug)]
enum Transport {
    None,
    Smtp {
        transport: lettre::AsyncSmtpTransport<lettre::Tokio1Executor>,
        from_address: String,
        from_name: Option<String>,
    },
}

pub struct Mail {
    settings: Arc<crate::settings::Settings>,
}

impl Mail {
    pub fn new(settings: Arc<crate::settings::Settings>) -> Self {
        Self { settings }
    }

    async fn get_transport(&self) -> Result<Transport, Box<dyn std::error::Error + Send + Sync>> {
        let settings = self.settings.get().await;

        match &settings.mail_mode {
            crate::settings::MailMode::None => Ok(Transport::None),
            crate::settings::MailMode::Smtp {
                host,
                port,
                username,
                password,
                use_tls,
                from_address,
                from_name,
            } => {
                let mut transport =
                    lettre::AsyncSmtpTransport::<lettre::Tokio1Executor>::builder_dangerous(host)
                        .port(*port)
                        .tls(if *use_tls {
                            lettre::transport::smtp::client::Tls::Required(
                                lettre::transport::smtp::client::TlsParametersBuilder::new(
                                    host.clone(),
                                )
                                .build_rustls()
                                .unwrap(),
                            )
                        } else {
                            lettre::transport::smtp::client::Tls::None
                        });

                if let Some(username) = username {
                    transport = transport.credentials(
                        lettre::transport::smtp::authentication::Credentials::new(
                            username.clone(),
                            password.clone().unwrap_or_default(),
                        ),
                    );
                }

                Ok(Transport::Smtp {
                    transport: transport.build(),
                    from_address: from_address.clone(),
                    from_name: from_name.clone(),
                })
            }
        }
    }

    pub async fn send(&self, destination: String, subject: String, body: String) {
        let transport = match self.get_transport().await {
            Ok(transport) => transport,
            Err(e) => {
                tracing::error!("failed to get mail transport: {:#?}", e);
                return;
            }
        };

        tracing::debug!(
            transport = ?transport,
            destination = destination,
            subject = subject,
            "sending email"
        );

        tokio::spawn(async move {
            let run = async || -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
                match transport {
                    Transport::None => Ok(()),
                    Transport::Smtp {
                        transport,
                        from_address,
                        from_name,
                    } => transport
                        .send(
                            lettre::message::Message::builder()
                                .subject(subject)
                                .to(lettre::message::Mailbox::new(None, destination.parse()?))
                                .from(lettre::message::Mailbox::new(
                                    from_name,
                                    from_address.parse()?,
                                ))
                                .header(lettre::message::header::ContentType::TEXT_HTML)
                                .body(body)?,
                        )
                        .await
                        .map(|_| ())
                        .map_err(|e| e.into()),
                }
            };

            match run().await {
                Ok(_) => tracing::debug!("email sent successfully"),
                Err(err) => tracing::error!("failed to send email: {:#?}", err),
            }
        });
    }
}
