use std::path::{Path, PathBuf};

use anyhow::{bail, Result};
use builder::{post_metadata::PostMetadata, TemplateMapping};
use strum_macros::Display;

const POSTS_DIR: &str = "../posts";
const PAGES_DIR: &str = "../pages";
const TEMPL_DIR: &str = "../templates";
const POST_LIST_FILE: &str = "../posts.html";

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

    if Path::new(POST_LIST_FILE).is_file() {
        std::fs::remove_file(POST_LIST_FILE).expect("should be able to remove post list file");
    }
}

// ----------------------------------------------------------------------------

fn build_web() -> Result<()> {
    let posts_metadata = collect_posts_metadata(Path::new(POSTS_DIR))?;

    build_post_list_template(&posts_metadata)?;

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

#[derive(Debug, Display)]
#[strum(serialize_all = "snake_case")]
enum Template {
    PostSummary,
    PostList,
    Post,
}

impl From<Template> for PathBuf {
    fn from(template_name: Template) -> Self {
        Self::from(format!("{TEMPL_DIR}/{template_name}.templ"))
    }
}

fn build_post_list_template(posts_metadata: &[PostMetadata]) -> Result<()> {
    let summary_elements = posts_metadata
        .iter()
        .map(populate_post_summary_template)
        .collect::<Result<Vec<String>, _>>()?
        .join("\n");

    let mapping: TemplateMapping = vec![("items", summary_elements)].into();

    let populated_template = populate_page_template(mapping, Template::PostList)?;

    std::fs::write(POST_LIST_FILE, populated_template)?;

    Ok(())
}

fn populate_post_summary_template(post_metadata: &PostMetadata) -> Result<String> {
    let mapping: TemplateMapping = vec![
        ("page_path", page_path(&post_metadata.file_name)),
        ("title", post_metadata.title.clone()),
        ("date", post_metadata.formatted_date()),
    ]
    .into();

    populate_page_template(mapping, Template::PostSummary)
}

fn populate_page_template(mapping: TemplateMapping, page_template: Template) -> Result<String> {
    let template_path: PathBuf = page_template.into();
    let template = std::fs::read_to_string(template_path)?;
    let populated_template = mapping.populate(template)?;

    Ok(populated_template)
}

fn page_path(file_name: &str) -> String {
    format!("./pages/{file_name}.html")
}
