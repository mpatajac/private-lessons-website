pub use template::TemplateMapping;

pub mod consts {
    pub const POSTS_DIR: &str = "../posts";
    pub const PAGES_DIR: &str = "../pages";
    pub const TEMPL_DIR: &str = "../templates";
    pub const POST_LIST_FILE: &str = "../posts.html";
}

#[derive(Debug, strum_macros::Display)]
#[strum(serialize_all = "snake_case")]
pub enum TemplateType {
    PostSummary,
    PostList,
    Post,
}

impl From<TemplateType> for std::path::PathBuf {
    fn from(template_type: TemplateType) -> Self {
        let templ_dir = consts::TEMPL_DIR;

        Self::from(format!("{templ_dir}/{template_type}.templ"))
    }
}

pub fn populate_page_template(
    mapping: TemplateMapping,
    template_type: TemplateType,
) -> anyhow::Result<String> {
    let template_path: std::path::PathBuf = template_type.into();
    let template = std::fs::read_to_string(template_path)?;
    let populated_template = mapping.populate(template)?;

    Ok(populated_template)
}

#[must_use]
pub fn page_route(file_name: &str) -> String {
    format!("./pages/{file_name}.html")
}

#[must_use]
pub fn page_path(file_name: &str) -> String {
    let pages_dir = consts::PAGES_DIR;

    format!("{pages_dir}/{file_name}.html")
}

#[derive(Debug, Clone)]
pub struct Post {
    pub metadata: post_metadata::PostMetadata,
    pub content: String,
}

pub mod storage {
    use std::{fs, io, path::Path};

    /// Safely removes a directory (checks for existance before removing)
    pub fn remove_dir(path: &str) -> io::Result<()> {
        if Path::new(path).is_dir() {
            fs::remove_dir_all(path)?;
        }

        Ok(())
    }

    /// Safely removes a file (checks for existance before removing)
    pub fn remove_file(path: &str) -> io::Result<()> {
        if Path::new(path).is_file() {
            fs::remove_file(path)?;
        }

        Ok(())
    }
}

pub mod post_metadata {
    use std::{
        fs::{self, File},
        io::{self, BufRead, BufReader},
        path::Path,
    };

    use chrono::{DateTime, Local, NaiveDate as Date};

    #[derive(Debug, Clone)]
    pub struct PostMetadata {
        pub path: String,
        pub file_name: String,
        pub title: String,
        pub date_created: Date,
    }

    impl PostMetadata {
        pub fn collect(file_path: &str) -> io::Result<Self> {
            Ok(Self {
                path: file_path.to_string(),
                file_name: Self::extract_file_name(file_path)?,
                title: Self::extract_post_title(file_path)?,
                date_created: Self::determine_date_created(file_path)?,
            })
        }

        #[must_use]
        pub fn formatted_date(&self) -> String {
            self.date_created.format("%d/%m/%Y").to_string()
        }

        fn extract_file_name(file_path: &str) -> io::Result<String> {
            let file_name = Path::new(file_path)
                .file_stem()
                .ok_or_else(|| io::Error::other("Could not extract file name"))?
                .to_string_lossy()
                .to_string();

            Ok(file_name)
        }

        fn determine_date_created(file_path: &str) -> io::Result<Date> {
            fs::metadata(file_path)?
                .created()
                .map(|system_time| Into::<DateTime<Local>>::into(system_time).date_naive())
        }

        fn extract_post_title(file_path: &str) -> io::Result<String> {
            let file = File::open(file_path)?;

            BufReader::new(file)
                .lines()
                .next()
                .ok_or_else(|| io::Error::other("Could not read first line from post"))?
                .map(|line| line.trim_start_matches("# ").trim().to_string())
        }
    }
}

pub mod markdown {
    pub fn to_html(content: &str) -> Result<String, String> {
        markdown::to_html_with_options(content, &markdown::Options::gfm())
    }
}

mod template {
    #[derive(Debug, PartialEq, Eq, strum_macros::Display)]
    pub enum Error {
        PlaceholderNotFound(Placeholder),
        MappingWithoutChange(TemplateItemMapping),
        LeftoverPlaceholders(Vec<String>),
    }

    impl std::error::Error for Error {}

    #[derive(Debug, PartialEq, Eq, Clone)]
    pub struct Placeholder(pub String);

    impl std::fmt::Display for Placeholder {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "{{{{ {} }}}}", self.0)
        }
    }

    impl Placeholder {
        pub fn new(value: &str) -> Self {
            Self(value.to_owned())
        }

        pub fn is_contained_in(&self, template: &str) -> bool {
            template.contains(&self.0)
        }
    }

    #[derive(Debug, PartialEq, Eq, Clone)]
    pub struct Replacement(pub String);

    impl Replacement {
        pub fn embed(&self, into: &str, instead_of: &str) -> String {
            into.replace(instead_of, &self.0)
        }
    }

    type TemplateItemMapping = (Placeholder, Replacement);

    #[derive(Debug)]
    pub struct TemplateMapping(pub Vec<TemplateItemMapping>);

    lazy_static::lazy_static! {
        static ref PLACEHOLDER_RE: regex::Regex = regex::Regex::new(r"\{\{ *[\w_]+ *\}\}").expect("placeholder ptrn should be valid");
    }

    impl TemplateMapping {
        #[allow(clippy::needless_pass_by_value)]
        fn apply_mapping(
            template_state: String,
            (placeholder, replacement): TemplateItemMapping,
        ) -> Result<String, Error> {
            if !placeholder.is_contained_in(&template_state) {
                return Err(Error::PlaceholderNotFound(placeholder));
            }

            let updated_template_state =
                replacement.embed(&template_state, &placeholder.to_string());

            if template_state == updated_template_state {
                return Err(Error::MappingWithoutChange((placeholder, replacement)));
            }

            Ok(updated_template_state)
        }

        pub fn populate(self, template: String) -> Result<String, Error> {
            let populated_template = self.0.into_iter().try_fold(template, Self::apply_mapping)?;

            if PLACEHOLDER_RE.is_match(&populated_template) {
                let leftover_placeholders = PLACEHOLDER_RE
                    .find_iter(&populated_template)
                    .map(|m| m.as_str().to_owned())
                    .collect();

                return Err(Error::LeftoverPlaceholders(leftover_placeholders));
            }

            Ok(populated_template)
        }
    }

    impl From<Vec<(&str, String)>> for TemplateMapping {
        fn from(items: Vec<(&str, String)>) -> Self {
            let mapping = |(k, v)| (Placeholder::new(k), Replacement(v));

            Self(items.into_iter().map(mapping).collect())
        }
    }
}

#[cfg(test)]
mod tests {
    // tests already panic on `assert`s, so we might as well use unwrap.
    #![allow(clippy::unwrap_used)]

    use super::*;

    #[test]
    fn test_md_to_html() {
        let md = "# Test post\n\nThis is a **test post** in which I test the capabilities of the [markdown conversion lib](https://crates.io/crates/markdown/1.0.0-alpha.16).";

        let expected_html = "<h1>Test post</h1>\n<p>This is a <strong>test post</strong> in which I test the capabilities of the <a href=\"https://crates.io/crates/markdown/1.0.0-alpha.16\">markdown conversion lib</a>.</p>";

        let received_html = markdown::to_html(md);

        assert!(received_html.is_ok());
        assert_eq!(received_html.unwrap(), expected_html);
    }

    // region: template

    #[test]
    fn test_templ_placeholder_to_string() {
        let placeholder = template::Placeholder::new("key");
        assert_eq!(placeholder.to_string(), "{{ key }}");
    }

    #[test]
    fn test_templ_placeholder_is_contained_in() {
        let placeholder = template::Placeholder::new("key");
        let templ = "Persistance is the {{ key }} to success";

        assert!(placeholder.is_contained_in(templ));
    }

    #[test]
    fn test_templ_replacement_embedding() {
        let placeholder = template::Placeholder::new("key");
        let replacement = template::Replacement(String::from("secret ingredient"));
        let templ = "Persistance is the {{ key }} to success";

        assert_eq!(
            replacement.embed(templ, &placeholder.to_string()),
            "Persistance is the secret ingredient to success"
        );
    }

    /// Helper function to extract data/actions common across template population tests.
    fn test_templ_populate(mapping: template::TemplateMapping) -> Result<String, template::Error> {
        let templ = String::from("Hello {{ receiver }}, {{ adjective }} to meet you!");

        mapping.populate(templ)
    }

    /// Correct template mapping for example in helper function
    fn default_template_mapping() -> template::TemplateMapping {
        use template::{Placeholder, Replacement};

        TemplateMapping(vec![
            (
                Placeholder::new("receiver"),
                Replacement(String::from("Matija")),
            ),
            (
                Placeholder::new("adjective"),
                Replacement(String::from("pleased")),
            ),
        ])
    }

    #[test]
    fn test_templ_populate_correct() {
        let populated_template = test_templ_populate(default_template_mapping());

        assert!(populated_template.is_ok());
        assert_eq!(
            populated_template.unwrap(),
            "Hello Matija, pleased to meet you!"
        );
    }

    #[test]
    fn test_templ_populate_no_placeholder() {
        use template::{Placeholder, Replacement};

        // add extra placeholder to default (correct) ones
        let missing_placeholder = Placeholder::new("missing");
        let mut mapping = default_template_mapping();
        mapping.0.push((
            missing_placeholder.clone(),
            Replacement(String::from("value")),
        ));

        let populated_template = test_templ_populate(mapping);

        assert!(populated_template.is_err());
        assert_eq!(
            populated_template.unwrap_err(),
            template::Error::PlaceholderNotFound(missing_placeholder)
        );
    }

    #[test]
    fn test_templ_populate_leftover_placeholder() {
        // remove placeholder from default (correct) ones
        let mut mapping = default_template_mapping();
        mapping.0.pop();

        let populated_template = test_templ_populate(mapping);

        assert!(populated_template.is_err());
        assert_eq!(
            populated_template.unwrap_err(),
            template::Error::LeftoverPlaceholders(vec![String::from("{{ adjective }}")])
        );
    }

    // endregion

    // region: post_metadata

    #[test]
    fn test_post_metadata_collection() {
        const TEST_FILE_PATH: &str = "./test_post_metadata_collection.md";
        const TEST_FILE_CONTENT: &str = "# Test post metadata collection

Paragraph inside test file.
";

        // setup: create dummy file
        std::fs::write(TEST_FILE_PATH, TEST_FILE_CONTENT).unwrap();

        let post_metadata = post_metadata::PostMetadata::collect(TEST_FILE_PATH);

        // cleanup: remove dummy file
        // NOTE: cleanup before assertions (in case of failed assertions)
        std::fs::remove_file(TEST_FILE_PATH).unwrap();

        assert!(post_metadata.is_ok());

        let post_metadata = post_metadata.unwrap();

        assert_eq!(post_metadata.path, TEST_FILE_PATH);
        assert_eq!(post_metadata.file_name, "test_post_metadata_collection");
        assert_eq!(post_metadata.title, "Test post metadata collection");
        assert_eq!(
            post_metadata.date_created,
            chrono::Local::now().date_naive()
        );
    }

    #[test]
    fn test_post_metadata_formatted_date() {
        let metadata = super::post_metadata::PostMetadata {
            date_created: chrono::NaiveDate::parse_from_str("13-02-2024", "%d-%m-%Y").unwrap(),
            // irrelevant to the test, so leave empty
            file_name: String::new(),
            path: String::new(),
            title: String::new(),
        };

        assert_eq!(metadata.formatted_date(), "13/02/2024");
    }
    // endregion
}
