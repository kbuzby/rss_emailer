use crate::rss_emailer_config::{Config, RssMailTo};
use lettre::smtp::authentication::Credentials;
use lettre::smtp::error::SmtpResult;
use lettre::SmtpClient;
use lettre::SmtpTransport;
use lettre::Transport;
use lettre_email::EmailBuilder;
use rss::Item;

pub struct FeedMailer {
    mailer: SmtpTransport,
    to: String,
    from: String,
}

impl FeedMailer {
    pub fn new(config: &Config) -> FeedMailer {
        let mailer = SmtpClient::new_simple(&config.smtp.server)
            .unwrap()
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
            .from(self.from.to_string())
            .subject(format!("[RSS:{}] {}", channel, title))
            .html(item.description.as_ref().unwrap())
            .build()
            .unwrap();

        return self.mailer.send(email.into());
    }
}
