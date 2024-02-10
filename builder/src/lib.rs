mod markdown {
    pub fn to_html(content: &str) -> Result<String, String> {
        markdown::to_html_with_options(content, &markdown::Options::gfm())
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
}
