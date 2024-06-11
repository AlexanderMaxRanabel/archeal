use colored::*;
use regex::Regex;
use scraper::{Html, Selector};

pub async fn localize_main_file(mut html_body: String, anchor_paths: Vec<String>, url: String) -> anyhow::Result<String> {
    let parsed_url = url::Url::parse(&url)?;
    let document = Html::parse_document(&html_body);

    let mut unordered_anchor_links: Vec<&str> = Vec::new();

    let anchor_selector = Selector::parse("a").unwrap();

    for link in document.select(&anchor_selector) {
        if let Some(href) = link.value().attr("href") {
            unordered_anchor_links.push(href);
        }
    }

    let re = Regex::new(r#"href="([^"]*)""#).unwrap();

    for path in anchor_paths {
        if let Some(mat) = re.find(&html_body) {
            let start = mat.start();
            let end = mat.end();
            //let matched_text = &[start..end];
            //let matched_text = &html_body[start..end];
            let new_href = format!(r#"href="{}""#, path);
            html_body.replace_range(start..end, &new_href);
        }
    }

    //TODO: Fix changer

    Ok(html_body)
}
