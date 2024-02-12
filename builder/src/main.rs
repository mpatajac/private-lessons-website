use std::path::{Path, PathBuf};

use anyhow::{bail, Result};
use builder::{html, post_metadata::PostMetadata, Post, TemplateMapping};
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

// TODO?: reorganize into `mod`s

fn build_web() -> Result<()> {
    let posts_metadata = collect_posts_metadata(Path::new(POSTS_DIR))?;

    build_post_list_template(&posts_metadata)?;
    build_post_templates(posts_metadata)?;

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
enum TemplateType {
    PostSummary,
    PostList,
    Post,
}

impl From<TemplateType> for PathBuf {
    fn from(template_type: TemplateType) -> Self {
        Self::from(format!("{TEMPL_DIR}/{template_type}.templ"))
    }
}

fn build_post_list_template(posts_metadata: &[PostMetadata]) -> Result<()> {
    let summary_elements = posts_metadata
        .iter()
        .map(populate_post_summary_template)
        .collect::<Result<Vec<String>, _>>()?
        .join("\n");

    let mapping: TemplateMapping = vec![("items", summary_elements)].into();

    let populated_template = populate_page_template(mapping, TemplateType::PostList)?;

    std::fs::write(POST_LIST_FILE, populated_template)?;

    Ok(())
}

fn populate_post_summary_template(post_metadata: &PostMetadata) -> Result<String> {
    let mapping: TemplateMapping = vec![
        ("page_path", page_route(&post_metadata.file_name)),
        ("title", post_metadata.title.clone()),
        ("date", post_metadata.formatted_date()),
    ]
    .into();

    populate_page_template(mapping, TemplateType::PostSummary)
}

fn build_post_templates(posts_metadata: Vec<PostMetadata>) -> Result<()> {
    posts_metadata
        .into_iter()
        .map(construct_post_data)
        .collect::<Result<Vec<Post>, _>>()?
        .into_iter()
        .try_for_each(build_post_page)
}

fn construct_post_data(post_metadata: PostMetadata) -> Result<Post> {
    let post_content = std::fs::read_to_string(&post_metadata.path)?;

    let cleaned_post_content = post_content
        .split_once('\n')
        .ok_or_else(|| std::io::Error::other("Could not split the first line from the others"))?
        .1
        .trim();

    match html::from_markdown(cleaned_post_content) {
        Err(msg) => bail!(msg),
        Ok(html) => Ok(Post {
            metadata: post_metadata,
            content: html,
        }),
    }
}

fn build_post_page(post: Post) -> Result<()> {
    let mapping: TemplateMapping = vec![
        ("title", post.metadata.title.clone()),
        ("date", post.metadata.formatted_date()),
        ("content", post.content),
    ]
    .into();

    let populated_template = populate_page_template(mapping, TemplateType::Post)?;

    std::fs::write(page_path(&post.metadata.file_name), populated_template)?;

    Ok(())
}

fn populate_page_template(mapping: TemplateMapping, template_type: TemplateType) -> Result<String> {
    let template_path: PathBuf = template_type.into();
    let template = std::fs::read_to_string(template_path)?;
    let populated_template = mapping.populate(template)?;

    Ok(populated_template)
}

fn page_route(file_name: &str) -> String {
    format!("./pages/{file_name}.html")
}

fn page_path(file_name: &str) -> String {
    format!("{PAGES_DIR}/{file_name}.html")
}
