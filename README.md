# RSS Emailer

This project provides the ability to read from configured RSS feeds and
email any new posts to configured recipients.

When RSS Emailer runs the recipients will receive an email with the subject
set to: `RSS:<Feed Title> - <Feed Item Name>`.

The body of the email will be the HTML content of of the `description` field of
the RSS entry.

## Configuring

The RSS Emailer is configured through a file located in ~/.config/rss_emailer/config

This file will be stored in plain text, so make sure to restrict permissions to avoid
access to your SMTP password.

The minimal configuration required is as follows, see config.example for all options.

```toml
[smtp]
server = "mail.example.com"
user = "me@example.com"
password = "password"

[rss_mail]
to = "rss@example.com"

# Feeds are a listing of key-value pairs of feed-name and URL to the feed
# feed-name can be any string key used to identify the feed, it the feed
# does not have a title then this will be used in place of the title.
# Example feed is listed below - remove this if you do not want to receive
# updates for hanselman.com
[feeds]
hanselman = "http://feeds.hanselman.com/ScottHanselman"
```

### Scheduling

The RSS Emailer has no configured scheduler. If you want to receive updates on a regular
basis you should configure it through a scheduling tool such as `cron` or Windows Task
Scheduler.
