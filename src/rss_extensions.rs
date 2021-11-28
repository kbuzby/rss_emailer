use rss::Channel;

pub trait ChannelExt {
    fn get_channel_name<'a>(&'a self, alternate: &'a str) -> &'a str;
}

impl ChannelExt for Channel {
    fn get_channel_name<'a>(&'a self, alternate: &'a str) -> &'a str {
        if !self.title.is_empty() {
            &self.title
        } else {
            alternate
        }
    }
}
