use clap::Parser;
use download::MediaCopy;
use log::info;
use rss::Channel;
use std::{error::Error, str::FromStr};
use tempfile::TempDir;

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

    /// Where to find the source of that file in Gpath. Note git paths are rooted in "" so no leading /
    #[arg(long, default_value = "static/mastodon.green/id.txt")]
    last_guid_git_path: String,

    /// Media path prefix for images. Note git paths are rooted in "" so no leading /
    #[arg(short, long, default_value = "static")]
    media_path_prefix: String,

    /// Post path prefix for markdown. Note git paths are rooted in "" so no leading /
    #[arg(short, long, default_value = "content/microposts")]
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


    /// Number of posts to read in this run
    #[arg(long, short, env = "NUM_POSTS", default_value = "1")]
    pub num_posts: usize,
    
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();

    let args = Args::parse();

    // let rss_str = include_str!("../rss/example01.rss");
    let rss_str = download::feed(&args.feed).await?;
    let channel = Channel::from_str(&rss_str).unwrap();

    let gh = github::Github::new(&args.github_token, &args.github_repo, &args.github_branch);
    let from = gh.get_last_guid(&args.last_guid_git_path).await?;

    let working_dir = TempDir::new().expect("creating temporary directory");

    for guid in channel.find_next_guids(&from).iter().take(args.num_posts) {
        // Locate the basics for this item:
        let item = channel.find_by_guid(guid).unwrap();
        let id = item.link().and_then(|url| url.split('/').last()).unwrap();

        // Prepare the markdown:
        let filename = markdown::post_filename(item.pub_date().unwrap(), id)?;
        let markdown_path = format!("{}/{filename}", &args.post_path);

        // Fetch any media:
        let media_map = item.download_all(&working_dir.path()).await?;
        let markdown = item.as_markdown(markdown::truncate_media_url)?;
        let path_map = media_map
            .apply(markdown::truncate_media_url)
            .apply(|u| format!("{}{u}", &args.media_path_prefix));

        // Convert into Github new content:
        let mut new_content: Vec<github::NewContent> = path_map
            .into_iter()
            .map(|(path, file)| github::NewContent::path(path, file))
            .collect();

        let md_content = github::NewContent::text(&markdown_path, &markdown);
        new_content.push(md_content);

        // ...including updating the next guid:
        let id_content = github::NewContent::text(&args.last_guid_git_path, guid.value());
        new_content.push(id_content);

        info!("{filename}");
        gh.commit(&format!("add {filename}"), &new_content).await?;
    }

    Ok(())
}
