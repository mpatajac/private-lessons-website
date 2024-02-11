use std::path::Path;

use anyhow::{bail, Result};
use builder::post_metadata::PostMetadata;

const POSTS_DIR: &str = "../posts";
const PAGES_DIR: &str = "../pages";
const POSTS_FILE: &str = "../posts.html";

fn main() {
    if let Err(error) = build_web() {
        eprintln!("ERROR: {error:#?}\n\nClearing up generated web...");
        cleanup_generated_web();
    }
}

fn cleanup_generated_web() {
    if Path::new(PAGES_DIR).is_dir() {
        std::fs::remove_dir_all(PAGES_DIR).expect("should be able to remove pages dir");
    }

    if Path::new(POSTS_FILE).is_file() {
        std::fs::remove_file(POSTS_FILE).expect("should be able to remove posts file");
    }
}

// ----------------------------------------------------------------------------

fn build_web() -> Result<()> {
    let posts_metadata = collect_posts_metadata(Path::new(POSTS_DIR))?;

    dbg!(posts_metadata);

    Ok(())
}

fn collect_posts_metadata(posts_dir: &Path) -> Result<Vec<PostMetadata>> {
    if !posts_dir.is_dir() {
        bail!(format!("{posts_dir:?} is not a directory"));
    }

    let mut collected_metadata = std::fs::read_dir(posts_dir)?
        .map(|file| PostMetadata::collect(&file?.path().to_string_lossy()))
        .collect::<Result<Vec<_>, _>>()
        .map_err(anyhow::Error::new)?;

    // sort files, starting from newest
    // TODO?: sort by `date_created` (to escape prefixing post file names)
    collected_metadata.sort_by(|left, right| right.file_name.cmp(&left.file_name));

    Ok(collected_metadata)
}
