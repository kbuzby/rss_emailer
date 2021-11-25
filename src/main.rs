use serde::Deserialize;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let content = reqwest::blocking::get("http://feeds.hanselman.com/ScottHanselman")?.text()?;
    let feed: Feed = quick_xml::de::from_str(&content).unwrap();
    for item in feed.channel.items {
        println!("{:#?}", item.title);
    }
    Ok(())
}

#[derive(Debug, Deserialize)]
struct Feed {
    channel: FeedChannel,
}

#[derive(Debug, Deserialize)]
struct FeedChannel {
    title: String,
    link: String,
    description: String,
    #[serde(rename = "item", default)]
    items: Vec<FeedItem>,
}

#[derive(Debug, Deserialize)]
struct FeedItem {
    title: String,
    link: String,
    description: String,
}
