use rss::{extension::Extension, Channel, Guid};
use std::str::FromStr;

use crate::feed::ChannelSurf;

mod feed;
mod markdown;
mod download;
use markdown::AsMarkdown;

fn main() {

    let rss_str = include_str!("../rss/example01.rss");
    let channel = Channel::from_str(rss_str).unwrap();

    let from: Guid = Guid {
        value: "https://mastodon.green/@d6y/109808565659434052".to_string(),
        permalink: true,
    };

    for guid in channel.find_next_guids(&from) {
        println!(
            "{:?}",
            channel
                .find_by_guid(guid)
                .unwrap()
                .as_markdown(markdown::truncate_media_url)
        )
    }
}

fn dump(channel: Channel) {
    for item in channel.items() {
        println!("{:?}", item.guid);
        println!("{:?}", item.link);
        println!("{:?}", item.description);
        println!("{:?}", item.pub_date);
        //println!("{:?}", item.extensions);
        for (ext_type, ext_map) in item.extensions.iter() {
            println!("{ext_type:?}");
            // vec<Extension>
            let xs: &Vec<Extension> = ext_map.get("content").unwrap();
            for x in xs {
                dbg!(x.attrs.get("type"));
                dbg!(x.attrs.get("url"));
                dbg!(x
                    .children
                    .get("description")
                    .and_then(|d| d[0].value.to_owned()));
            }
        }
    }
}
