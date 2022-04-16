use crate::rss_emailer_config::{Config, RssMailTo};
use lettre::smtp::authentication::Credentials;
use lettre::smtp::client::net::DEFAULT_TLS_PROTOCOLS;
use lettre::smtp::error::SmtpResult;
use lettre::ClientSecurity;
use lettre::ClientTlsParameters;
use lettre::SmtpClient;
use lettre::SmtpTransport;
use lettre::Transport;
use lettre_email::EmailBuilder;
use lettre_email::Mailbox;
use native_tls::TlsConnector;
use rss::Item;

pub struct FeedMailer {
    mailer: SmtpTransport,
    to: String,
    from: String,
}

impl FeedMailer {
    pub fn new(config: &Config) -> FeedMailer {
        let mut tls_builder = TlsConnector::builder();
        tls_builder.min_protocol_version(Some(DEFAULT_TLS_PROTOCOLS[0]));

        let domain = &config.smtp.server;
        let tls_parameters =
            ClientTlsParameters::new(domain.to_string(), tls_builder.build().unwrap());

        let port = match config.smtp.port {
            Some(x) => x,
            _ => 465,
        };
        let client = SmtpClient::new(
            (domain.to_string(), port),
            ClientSecurity::Required(tls_parameters),
        )
        .unwrap();

        let mailer = client
            .credentials(Credentials::new(
                config.smtp.user.to_string(),
                config.smtp.password.to_string(),
            ))
            .transport();

        let from = match &config.rss_mail.from {
            Some(x) => x,
            _ => &config.smtp.user,
        }
        .to_string();

        let to = match &config.rss_mail.to {
            RssMailTo::Single(x) => x.to_string(),
            RssMailTo::Multiple(x) => x.join(";"),
        };

        FeedMailer { mailer, to, from }
    }

    pub fn send_email(&mut self, channel: &str, item: &Item) -> SmtpResult {
        let title = item.title.as_ref().unwrap();
        let email = EmailBuilder::new()
            .to(self.to.to_string())
            .from(self.from.to_string().parse::<Mailbox>().unwrap())
            .subject(format!("[RSS:{}] {}", channel, title))
            .html(item.description.as_ref().unwrap())
            .build()
            .unwrap();

        self.mailer.send(email.into())
    }
}
