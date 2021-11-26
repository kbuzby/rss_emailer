mod feed_mailer;
mod rss_mailer_config;

use feed_mailer::FeedMailer;
use rss::Channel;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let content = reqwest::blocking::get("http://feeds.hanselman.com/ScottHanselman")?.bytes()?;
    let channel = Channel::read_from(&content[..])?;
    let config = rss_mailer_config::read_from_file("/etc/rss_mailer/config");
    let mut mailer = FeedMailer::new(&config);
    mailer
        .send_email(&channel.title, &channel.items[0])
        .unwrap();
    Ok(())
}
