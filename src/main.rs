use download::MediaCopy;
use rss::{extension::Extension, Channel, Guid};
use std::{error::Error, path::Path, str::FromStr};

use crate::feed::ChannelSurf;

mod download;
mod feed;
mod markdown;
use markdown::AsMarkdown;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let rss_str = include_str!("../rss/example01.rss");
    let channel = Channel::from_str(rss_str).unwrap();

    let from: Guid = Guid {
        value: "https://mastodon.green/@d6y/109818375938647316".to_string(),
        permalink: true,
    };

    let working_dir = Path::new("./tmp");

    for guid in channel.find_next_guids(&from).iter().take(1) {
        let item = channel.find_by_guid(guid).unwrap();
        let id = item.link().and_then(|url| url.split("/").last()).unwrap();
        let filename = markdown::post_filename(item.pub_date().unwrap(), id)?;

        let media_map = item.download_all(working_dir).await?;

        println!("{filename}");
        println!("{:?}", item.as_markdown(markdown::truncate_media_url));
        println!("{media_map:?}");
    }

    Ok(())
}

fn _dump(channel: Channel) {
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
