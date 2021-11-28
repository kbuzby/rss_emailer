use log::{debug, info};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::Path;

#[derive(Debug, Serialize, Deserialize)]
pub struct FeedHistory {
    path: String,
    feed_items: HashMap<String, HashSet<String>>,
}

impl FeedHistory {
    pub fn new(path: &str) -> FeedHistory {
        FeedHistory {
            path: path.to_string(),
            feed_items: HashMap::new(),
        }
    }

    pub fn get_or_create(path: &str) -> FeedHistory {
        debug!("Attempting to get or create feed history.");
        match FeedHistory::get_from_file(path) {
            Some(history) => history,
            _ => {
                info!("Creating new feed history.");
                FeedHistory::new(path)
            }
        }
    }

    fn get_from_file(path: &str) -> Option<FeedHistory> {
        if Path::new(path).exists() {
            info!("Reading feed history from {}.", path);
            let config_file = fs::read_to_string(path);
            return Some(serde_json::from_str::<FeedHistory>(&config_file.unwrap()).unwrap());
        }
        info!("Feed history does not exist.");
        return None;
    }

    pub fn save_to_disk(&self) -> Result<(), Box<dyn std::error::Error>> {
        info!("Saving feed history to {}.", &self.path);
        let string = serde_json::to_string_pretty(self)?;
        let _ = fs::write(&self.path, string)?;
        Ok(())
    }

    pub fn item_sent(&self, feed_name: &str, item_name: &str) -> bool {
        match self.feed_items.get(feed_name) {
            Some(items) => items.contains(item_name),
            _ => false,
        }
    }

    pub fn track_item(&mut self, feed_name: &str, item_name: &str) {
        self.ensure_feed_history_exists(feed_name);
        self.feed_items
            .get_mut(feed_name)
            .unwrap()
            .insert(item_name.to_string());
    }

    fn ensure_feed_history_exists(&mut self, feed_name: &str) {
        if !self.feed_items.contains_key(feed_name) {
            self.feed_items
                .insert(feed_name.to_string(), HashSet::new());
        }
    }
}
