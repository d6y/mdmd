use chrono::{DateTime, ParseError};
use rss::Item;

trait AsMarkdown {
    fn as_markdown(&self) -> Result<String, ParseError>;
}

impl AsMarkdown for Item {
    fn as_markdown(&self) -> Result<String, ParseError> {
        let title = title_date(self.pub_date().unwrap())?;
        let msg = self.description().unwrap();
        let instance = "mastodon.green";
        let url = self.link().unwrap();
        let date = formal_date(self.pub_date().unwrap())?;
        Ok(format!(
            r#"--
title: {title}
instance: {instance}
toot_url: {url}
date: {date}
--

{msg}
"#
        ))
    }
}

fn title_date(pub_date: &str) -> Result<String, ParseError> {
    let title_format = "%a %d %b %Y %H:%M"; // Tue 12 Dec 2006 11:02
    DateTime::parse_from_rfc2822(pub_date).map(|dt| dt.format(title_format).to_string())
}

fn formal_date(pub_date: &str) -> Result<String, ParseError> {
    DateTime::parse_from_rfc2822(pub_date).map(|dt| dt.to_rfc3339().to_string())
}

fn filename_date(pub_date: &str) -> Result<String, ParseError> {
    let filename_format = "%Y-%m-%d"; // 2005-12-30
    DateTime::parse_from_rfc2822(pub_date).map(|dt| dt.format(filename_format).to_string())
}

// 2006-12-12-tweet-996943.md

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

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
toot_url: https://mastodon.green/@d6y/109808565659434052
date: 2023-02-04T21:22:20+00:00
--

<p>A visit to the ASMR exhibit at the Design Museum. Yes, of course there was a Bob Ross room (as part of the unintentional ASMR section of the exhibit).</p><p><a href="https://designmuseum.org/exhibitions/weird-sensation-feels-good-the-world-of-asmr" target="_blank" rel="nofollow noopener noreferrer"><span class="invisible">https://</span><span class="ellipsis">designmuseum.org/exhibitions/w</span><span class="invisible">eird-sensation-feels-good-the-world-of-asmr</span></a></p>

![An area partly enclosed by cream coloured soft walls, bending in like hands or waves. There are three large TV screens showing ASMR works. Groups of people, in twos and threes, sit on the soft cushioning wearing headsets, the cables stretching to the screens. No one is wearing shoes.](https://files.mastodon.green/media_attachments/files/109/808/524/409/930/443/original/38da0da8e5badfdc.jpeg)

![An entry in the programme, describing a video: “ASMR can be triggered by watching someone explain something.
In 1988, the Icelandic artist Björk was the lead singer of The Sugarcubes. In this film, which was part of a documentary made about the band, Bork describes how a television works. Her softly spoken description is accompanied by a close-up shot of her finger tracing the inside of a cathode-ray tube television set. 
The success of an ASM work is often determined by the relationship a viewer has to the ASMRtist through the screen. In this film, Björk's explanation is both generous and empathetic.”](https://files.mastodon.green/media_attachments/files/109/808/524/733/756/242/original/4f4f643fae86839f.jpeg)
"#;

        assert_eq!(expected, item.as_markdown().unwrap());
    }
}
