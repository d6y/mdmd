use async_trait::async_trait;
use std::error::Error;
use std::path::{Path, PathBuf};

use tokio::fs::File;
use tokio::io::AsyncWriteExt;

use crate::feed::ItemSurf;
use reqwest::Client;
use rss::Item;

#[derive(Debug)]
pub struct LocalMedia {
    urls: Vec<String>,
    local_files: Vec<PathBuf>,
}

impl<'a> IntoIterator for &'a LocalMedia {
    type Item = (&'a String, &'a PathBuf);
    type IntoIter = std::iter::Zip<std::slice::Iter<'a, String>, std::slice::Iter<'a, PathBuf>>;

    fn into_iter(self) -> Self::IntoIter {
        self.urls.iter().zip(self.local_files.iter())
    }
}

impl LocalMedia {
    fn new() -> LocalMedia {
        LocalMedia {
            urls: vec![],
            local_files: vec![],
        }
    }

    fn push(&mut self, url: &str, local_file: &Path) {
        self.urls.push(url.to_owned());
        self.local_files.push(local_file.to_owned());
    }

    pub fn apply<F: Fn(&str) -> String>(&self, f: F) -> LocalMedia {
        let new_urls = self.urls.iter().map(|u| f(u)).collect();
        LocalMedia {
            urls: new_urls,
            local_files: self.local_files.to_owned(),
        }
    }
}

#[async_trait]
pub trait MediaCopy {
    async fn download_all(&self, working_dir: &Path) -> Result<LocalMedia, Box<dyn Error>>;
}

#[async_trait]
impl MediaCopy for Item {
    async fn download_all(&self, working_dir: &Path) -> Result<LocalMedia, Box<dyn Error>> {
        let mut map = LocalMedia::new();
        let client = Client::new();

        for media in self.medias() {
            let media_url = media.attrs.get("url").unwrap();
            let local_file = working_dir.join(Path::new(media_url).file_name().unwrap());

            let response = client.get(media_url).send().await?;

            // We probably have enough memory to read our files into RAM:
            let bytes = response.bytes().await?;

            let mut file = File::create(&local_file).await?;
            file.write_all(&bytes).await?;

            map.push(media_url, &local_file);
        }

        Ok(map)
    }
}
