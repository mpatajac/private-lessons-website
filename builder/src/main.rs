use std::path::Path;

use anyhow::{bail, Result};
use builder::{
    consts, markdown, post_metadata::PostMetadata, storage, Post, TemplateMapping, TemplateType,
};

fn main() {
    if let Err(error) = build_web() {
        eprintln!("ERROR: {error:#?}\n\nClearing up generated web...");
        cleanup_generated_web();
    }
}

fn build_web() -> Result<()> {
    let posts_metadata = collect_posts_metadata(Path::new(consts::POSTS_DIR))?;

    post_list_template::build(&posts_metadata)?;
    post_template::build_all(posts_metadata)?;

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

mod post_list_template {
    #[allow(clippy::wildcard_imports)]
    use super::*;

    pub fn build(posts_metadata: &[PostMetadata]) -> Result<()> {
        let summary_elements = posts_metadata
            .iter()
            .map(populate_post_summary_template)
            .collect::<Result<Vec<String>, _>>()?
            .join("\n");

        let mapping: TemplateMapping = vec![("items", summary_elements)].into();

        let populated_template = builder::populate_page_template(mapping, TemplateType::PostList)?;

        std::fs::write(consts::POST_LIST_FILE, populated_template)?;

        Ok(())
    }

    fn populate_post_summary_template(post_metadata: &PostMetadata) -> Result<String> {
        let mapping: TemplateMapping = vec![
            ("page_path", builder::page_route(&post_metadata.file_name)),
            ("title", post_metadata.title.clone()),
            ("date", post_metadata.formatted_date()),
        ]
        .into();

        builder::populate_page_template(mapping, TemplateType::PostSummary)
    }
}

mod post_template {
    #[allow(clippy::wildcard_imports)]
    use super::*;

    pub fn build_all(posts_metadata: Vec<PostMetadata>) -> Result<()> {
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

        match markdown::to_html(cleaned_post_content) {
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

        let populated_template = builder::populate_page_template(mapping, TemplateType::Post)?;

        std::fs::write(
            builder::page_path(&post.metadata.file_name),
            populated_template,
        )?;

        Ok(())
    }
}

fn cleanup_generated_web() {
    storage::remove_dir(consts::PAGES_DIR).expect("should be able to remove pages dir");
    storage::remove_file(consts::POST_LIST_FILE).expect("should be able to remove post list file");
}
