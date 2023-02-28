use clap::Parser;
use download::MediaCopy;
use rss::{extension::Extension, Channel, Guid};
use std::{error::Error, path::Path, str::FromStr};

use crate::feed::ChannelSurf;

mod download;
mod feed;
mod github;
mod markdown;
use markdown::AsMarkdown;

#[derive(Parser, Debug)]
struct Args {
    /// RSS Feed to chec
    #[arg(short, long, default_value = "http://mastodon.green/@d6y.rss")]
    feed: String,

    /// Where to find the last RSS GUID we processed
    #[arg(
        long,
        default_value = "https://richard.dallaway.com/mastodon.green/id.txt"
    )]
    last_guid_url: String,

    /// Where to find the source of that file in GIT
    #[arg(long, default_value = "/static/mastodon.green/id.txt")]
    last_guid_git_path: String,

    /// Media path prefix for images
    #[arg(short, long, default_value = "/static")]
    media_path_prefix: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let rss_str = include_str!("../rss/example01.rss");
    let channel = Channel::from_str(rss_str).unwrap();

    let from: Guid = download::last_guid(&args.last_guid_url).await?;

    let working_dir = Path::new("./tmp");

    for guid in channel.find_next_guids(&from).iter().take(1) {
        let item = channel.find_by_guid(guid).unwrap();
        let id = item.link().and_then(|url| url.split('/').last()).unwrap();
        let filename = markdown::post_filename(item.pub_date().unwrap(), id)?;

        let media_map = item.download_all(working_dir).await?;
        let markdown = item.as_markdown(markdown::truncate_media_url)?;
        let path_map = media_map
            .apply(markdown::truncate_media_url)
            .apply(|u| format!("{}{u}", &args.media_path_prefix));

        println!("{filename}");
        println!("{markdown}");
        println!("{path_map:?}");
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
