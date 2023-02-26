use std::path::{Path, PathBuf};
use std::error::Error;

use rss::Item;
pub struct LocalMedia {
    urls: Vec<String>,
    local_files: Vec<PathBuf>,
}

pub trait MediaCopy {
    fn download_all(&self, working_dir: Path) -> Result<LocalMedia, Box<dyn Error>>;
}

impl MediaCopy for Item {
}

