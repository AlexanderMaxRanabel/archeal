use crate::{fetch_webpage, order_links};
use std::{fs, io::prelude::*};

use scraper::{Html, Selector};

pub async fn depth_fetch_page(
    html_body: String,
    current_path: String,
    url: String,
) -> anyhow::Result<()> {
    let parsed_url = url::Url::parse(&url)?;
    let document = Html::parse_document(&html_body);

    let mut unordered_anchor_links: Vec<&str> = Vec::new();
    let mut anchor_links: Vec<String> = Vec::new();

    let anchor_selector = Selector::parse("a").unwrap();

    for link in document.select(&anchor_selector) {
        if let Some(href) = link.value().attr("href") {
            unordered_anchor_links.push(href);
        }
    }

    for sublink in unordered_anchor_links {
        if sublink.starts_with("https://") || sublink.starts_with("http://") {
            anchor_links.push(sublink.to_string());
        } else {
            let host = parsed_url.host_str();
            let parts: Vec<&str> = host.expect("Failed the transformation").split('.').collect();
            let root = if parts.len() >= 2 {
                format!("{}.{}", parts[parts.len() - 2], parts[parts.len() - 1])
            } else {
                host.expect("Failed the transformation").to_string()
            };
            let new_sublink = format!("https://{}{}", &root, sublink);
            anchor_links.push(new_sublink);
        }
    }

    for sublink in anchor_links {
        println!("{}", sublink);
        let sublink_html_body = fetch_webpage::fetch_page(sublink.to_string()).await?;

        let sublink_parsed_url = url::Url::parse(&sublink)?;
        let sublink_domain = parsed_url.domain().unwrap_or("unknown domain");

        let sublink_sanitized_domain = sublink_domain
            .chars()
            .filter(|&c| c.is_ascii_alphanumeric() || c == '-' || c == '.')
            .collect::<String>();

        let sublink_path = sublink_parsed_url.path().trim_start_matches('/');

        let sublink_path_with_dashes = sublink_path.replace("/", "-");

        let sublink_html_path = format!(
            "{}/{}-{}.html",
            current_path, sublink_sanitized_domain, sublink_path_with_dashes
        );

        let sublink_html = fs::File::create(sublink_html_path.clone());

        sublink_html
            .expect("Cannot write to this file")
            .write_all(sublink_html_body.as_bytes())?;
    }
    Ok(())
}
