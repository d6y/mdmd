use rss::Item;

trait AsMarkdown {
    fn as_markdown(&self) -> String;
}

impl AsMarkdown for Item {
    fn as_markdown(&self) -> String {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::feed::ChannelSurf;
    use rss::{Channel, Guid};
    use std::str::FromStr;

    const RSS_STR: &str = include_str!("../rss/example01.rss");

    #[test]
    fn test_find_next_from_older() {
        let channel = Channel::from_str(RSS_STR).unwrap();

        // We know there are two GUIDs after this one in the example01 RSS feed
        let from: Guid = Guid {
            value: "https://mastodon.green/@d6y/109818375938647316".to_string(),
            permalink: true,
        };

        let item = channel.find_by_guid(&from).unwrap();

        let expected = String::from("");

        assert_eq!(expected, item.as_markdown());
    }
}
