use crate::fetch_webpage;
use crate::order_weblinks;
use std::{fs, io::prelude::*};

pub async fn depth_fetch_page(
    html_body: String,
    current_path: String,
    url: String,
) -> anyhow::Result<()> {
    let parsed_url = url::Url::parse(&url)?;

    let mut anchor_links: Vec<String> = Vec::new();
    anchor_links = order_weblinks::order_possible_links(anchor_links, html_body.clone(), url.clone()).await?;

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
