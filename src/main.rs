mod feed_history;
mod feed_mailer;
mod rss_emailer_config;
mod rss_extensions;

use feed_history::FeedHistory;
use feed_mailer::FeedMailer;
use log::{debug, error, info};
use rss::{Channel, Item};
use rss_emailer_config::FeedDefinition;
use rss_extensions::ChannelExt;
use simplelog::*;
use std::error::Error;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn Error>> {
    CombinedLogger::init(vec![TermLogger::new(
        LevelFilter::Info,
        Config::default(),
        TerminalMode::Mixed,
        ColorChoice::Auto,
    )])
    .unwrap();

    let config = get_config()?;
    let mut mailer = FeedMailer::new(&config);
    let mut history = get_history()?;
    for (feed_name, feed_def) in config.feeds {
        if let FeedDefinition::Simple(feed_link) = feed_def {
            if let Err(e) = send_feed_items(feed_link, &feed_name, &mut history, &mut mailer) {
                error!("Failure sending items for feed {}. {}.", &feed_name, e)
            };
        }
    }
    match history.save_to_disk() {
        Ok(_) => debug!("History successfully saved."),
        Err(e) => error!("History unable to be saved. Error: {}.", e),
    };
    Ok(())
}

fn send_feed_items(
    feed_link: String,
    feed_name: &str,
    history: &mut FeedHistory,
    mailer: &mut FeedMailer,
) -> Result<(), Box<dyn Error>> {
    info!("Sending new items for feed: {}.", feed_name);
    let channel = get_channel_for_link(feed_link)?;
    let title = channel.get_channel_name(feed_name);
    for item in &channel.items {
        send_if_new(history, mailer, title, item);
    }
    Ok(())
}

fn get_channel_for_link(feed_link: String) -> Result<Channel, Box<dyn Error>> {
    debug!("Retrieving channel from {}.", feed_link);
    let content = reqwest::blocking::get(feed_link)?.bytes()?;
    Ok(Channel::read_from(&content[..])?)
}

fn send_if_new(history: &mut FeedHistory, mailer: &mut FeedMailer, feed_name: &str, item: &Item) {
    let item_title = item.title.as_ref().unwrap();
    if !history.item_sent(feed_name, item_title) {
        info!("Sending new item {} for feed {}.", item_title, feed_name);
        if mailer.send_email(feed_name, item).is_ok() {
            history.track_item(feed_name, item_title);
        } else {
            error!(
                "Unable to send item {} for feed {}. Will try again next time.",
                item_title, feed_name
            );
        }
    } else {
        debug!("Item {} in feed {} already sent.", item_title, feed_name);
    }
}

fn get_config() -> Result<rss_emailer_config::Config, Box<dyn Error>> {
    let config_path = get_config_path().unwrap();
    let config_path_str = config_path.to_str().unwrap();
    rss_emailer_config::read_from_file(config_path_str)
}

fn get_config_path() -> Option<PathBuf> {
    debug!("Getting config path from home directory.");
    Some(home::home_dir()?.join(".config/rss_emailer/config"))
}

fn get_history() -> Result<FeedHistory, Box<dyn Error>> {
    let history_path = get_history_path().unwrap();
    let history_path_str = history_path.to_str().unwrap();
    Ok(FeedHistory::get_or_create(history_path_str))
}

fn get_history_path() -> Option<PathBuf> {
    debug!("Getting history path from home directory.");
    Some(home::home_dir()?.join(".config/rss_emailer/history"))
}
