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

    /// Post path prefix for markdown
    #[arg(short, long, default_value = "/content/microposts")]
    post_path: String,

    /// Github bearer token
    #[arg(long, env = "GITHUB_TOKEN", hide_env_values = true)]
    pub github_token: String,

    /// Github repository in the form "user/repo"
    #[arg(long, env = "GITHUB_REPO")]
    pub github_repo: String,

    /// Github repository branch
    #[arg(long, env = "GITHUB_BRANCH", default_value = "main")]
    pub github_branch: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let rss_str = include_str!("../rss/example01.rss");
    let channel = Channel::from_str(rss_str).unwrap();

    let from: Guid = download::last_guid(&args.last_guid_url).await?;

    let working_dir = Path::new("./tmp");

    let gh = github::Github::new(&args.github_token, &args.github_repo, &args.github_branch);

    for guid in channel.find_next_guids(&from).iter().take(1) {
        let item = channel.find_by_guid(guid).unwrap();
        let id = item.link().and_then(|url| url.split('/').last()).unwrap();

        let filename = markdown::post_filename(item.pub_date().unwrap(), id)?;
        let markdown_path = format!("{}/{filename}", &args.post_path);

        let media_map = item.download_all(working_dir).await?;
        let markdown = item.as_markdown(markdown::truncate_media_url)?;
        let path_map = media_map
            .apply(markdown::truncate_media_url)
            .apply(|u| format!("{}{u}", &args.media_path_prefix));

        let mut new_content: Vec<github::NewContent> = path_map
            .into_iter()
            .map(|(path, file)| github::NewContent::path(path, file))
            .collect();

        let md_content = github::NewContent::text(&markdown_path, &markdown);
        new_content.push(md_content);

        println!("{filename}");
        println!("{new_content:?}");
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
