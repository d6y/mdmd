use chrono::{DateTime, ParseError};
use rss::Item;

trait AsMarkdown {
    fn as_markdown(&self) -> Result<String, ParseError>;
}

impl AsMarkdown for Item {
    fn as_markdown(&self) -> Result<String, ParseError> {
        let date = title_date(self.pub_date().unwrap())?;
        let msg = self.description().unwrap();
        Ok(format!("# {date}\n\n{msg}"))
    }
}

fn title_date(pub_date: &str) -> Result<String, ParseError> {
    let title_format = "%a %d %b %Y %H:%M"; // Tue 12 Dec 2006 11:02
    DateTime::parse_from_rfc2822(pub_date).map(|dt| dt.format(title_format).to_string())
}

fn filename_date(pub_date: &str) -> Result<String, ParseError> {
    let filename_format = "%Y-%m-%d"; // 2005-12-30
    DateTime::parse_from_rfc2822(pub_date).map(|dt| dt.format(filename_format).to_string())
}

// 2006-12-12-tweet-996943.md

#[cfg(test)]
mod tests {
    use super::*;
    use crate::feed::ChannelSurf;
    use rss::{Channel, Guid};
    use std::str::FromStr;

    #[test]
    fn test_convert_to_title_date() {
        assert_eq!(
            "Sat 04 Feb 2023 19:01",
            title_date("Sat, 04 Feb 2023 19:01:20 +0000").unwrap()
        );
    }

    #[test]
    fn test_convert_to_filename_date() {
        assert_eq!(
            "2023-02-04",
            filename_date("Sat, 04 Feb 2023 19:01:20 +0000").unwrap()
        );
    }

    #[test]
    fn test_convert_item_to_markdown() {
        const RSS_STR: &str = include_str!("../rss/example01.rss");
        let channel = Channel::from_str(RSS_STR).unwrap();

        let from: Guid = Guid {
            value: "https://mastodon.green/@d6y/109808565659434052".to_string(),
            permalink: true,
        };

        let item = channel.find_by_guid(&from).unwrap();

        let expected = r#"--
title: Sat 04 Feb 2023 21:22
instance: mastodon.green
toot_url: https://mastodon.social/users/d6y/statuses/103498823626731219
date: 2023-02-04T21:39:35Z
--

<p>A visit to the ASMR exhibit at the Design Museum. Yes, of course there was a Bob Ross room (as part of the unintentional ASMR section of the exhibit).</p><p><a href="https://designmuseum.org/exhibitions/weird-sensation-feels-good-the-world-of-asmr" target="_blank" rel="nofollow noopener noreferrer"><span class="invisible">https://</span><span class="ellipsis">designmuseum.org/exhibitions/w</span><span class="invisible">eird-sensation-feels-good-the-world-of-asmr</span></a></p>"#;

        assert_eq!(expected, item.as_markdown().unwrap());
    }
}
