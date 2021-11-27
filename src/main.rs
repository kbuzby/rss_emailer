mod feed_history;
mod feed_mailer;
mod rss_emailer_config;

use feed_history::FeedHistory;
use feed_mailer::FeedMailer;
use rss::{Channel, Item};
use rss_emailer_config::FeedDefinition;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    if let Some(config_path) = get_config_path() {
        let config = rss_emailer_config::read_from_file(config_path.to_str().unwrap());
        let mut mailer = FeedMailer::new(&config);
        let mut history = FeedHistory::get_or_create(get_history_path().unwrap().to_str().unwrap());
        for (feed_name, feed_def) in config.feeds {
            if let FeedDefinition::Simple(feed_link) = feed_def {
                let content = reqwest::blocking::get(feed_link)?.bytes()?;
                let channel = Channel::read_from(&content[..])?;
                let title = if !channel.title.is_empty() {
                    &channel.title
                } else {
                    &feed_name
                };
                for item in channel.items {
                    send_if_new(&mut history, &mut mailer, title, &item);
                }
            } else if let FeedDefinition::Detailed(_feed_detail) = feed_def {
                /* TODO */
            }
        }
        let _ = history.save_to_disk()?;
        return Ok(());
    }
    panic!("Unable to get config file.");
}

fn send_if_new(history: &mut FeedHistory, mailer: &mut FeedMailer, feed_name: &str, item: &Item) {
    let item_title = item.title.as_ref().unwrap();
    if !history.item_sent(feed_name, item_title) {
        if let Ok(_) = mailer.send_email(feed_name, item) {
            history.track_item(feed_name, item_title);
        }
    }
}

fn get_config_path() -> Option<PathBuf> {
    if let Some(home_path) = home::home_dir() {
        return Some(home_path.join(".config/rss_emailer/config"));
    }
    return None;
}

fn get_history_path() -> Option<PathBuf> {
    if let Some(home_path) = home::home_dir() {
        return Some(home_path.join(".config/rss_emailer/history"));
    }
    return None;
}
