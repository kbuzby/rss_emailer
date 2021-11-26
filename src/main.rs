mod feed_mailer;
mod rss_emailer_config;

use feed_mailer::FeedMailer;
use rss::Channel;
use rss_emailer_config::FeedDefinition;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    if let Some(config_path) = get_config_path() {
        let config = rss_emailer_config::read_from_file(config_path.to_str().unwrap());
        let mut mailer = FeedMailer::new(&config);
        for (feed_name, feed_def) in config.feeds {
            if let FeedDefinition::Simple(feed_link) = feed_def {
                let content = reqwest::blocking::get(feed_link)?.bytes()?;
                let channel = Channel::read_from(&content[..])?;
                let title = if !channel.title.is_empty() {
                    &channel.title
                } else {
                    &feed_name
                };
                if let Ok(_) = mailer.send_email(title, &channel.items[0]) {}
            } else if let FeedDefinition::Detailed(_feed_detail) = feed_def {
                /* TODO */
            }
        }
        return Ok(());
    }
    panic!("Unable to get config file.");
}

fn get_config_path() -> Option<PathBuf> {
    if let Some(home_path) = home::home_dir() {
        return Some(home_path.join(".rss_emailer"));
    }
    return None;
}
