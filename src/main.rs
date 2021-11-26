mod feed_mailer;
mod rss_emailer_config;

use feed_mailer::FeedMailer;
use rss::Channel;
use rss_emailer_config::FeedDefinition;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = rss_emailer_config::read_from_file("/etc/rss_mailer/config");
    let mut mailer = FeedMailer::new(&config);
    for (_feed_name, feed_def) in config.feeds {
        if let FeedDefinition::Simple(feed_link) = feed_def {
            let content = reqwest::blocking::get(feed_link)?.bytes()?;
            let channel = Channel::read_from(&content[..])?;
            if let Ok(_) = mailer.send_email(&channel.title, &channel.items[0]) {}
        } else if let FeedDefinition::Detailed(_feed_detail) = feed_def {
            /* TODO */
        }
    }
    Ok(())
}
