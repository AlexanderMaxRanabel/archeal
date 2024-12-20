use colored::*;
use scraper::{Html, Selector};
use std::{
    collections::HashSet,
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
};

pub async fn order_possible_links(
    mut anchor_links: Vec<String>,
    html_body: String,
    url: String,
    skip_list_path: String,
) -> anyhow::Result<Vec<String>> {
    let parsed_url = url::Url::parse(&url)?;
    let document = Html::parse_document(&html_body);

    let mut unordered_anchor_links: Vec<&str> = Vec::new();

    let anchor_selector = Selector::parse("a").unwrap();

    for link in document.select(&anchor_selector) {
        if let Some(href) = link.value().attr("href") {
            unordered_anchor_links.push(href);
        }
    }

    if skip_list_path == "None" {
        for sublink in unordered_anchor_links {
            if sublink.starts_with("https://") || sublink.starts_with("http://") {
                anchor_links.push(sublink.to_string());
            } else {
                let host = parsed_url.host_str();
                let parts: Vec<&str> = host
                    .expect("Failed the transformation")
                    .split('.')
                    .collect();
                let root = if parts.len() >= 2 {
                    format!("{}.{}", parts[parts.len() - 2], parts[parts.len() - 1])
                } else {
                    host.expect("Failed the transformation").to_string()
                };
                let new_sublink = format!("https://{}/{}", &root, sublink);
                anchor_links.push(new_sublink);
            }
        }
    } else {
        println!("{}: Skip lists, while helping with unwanted archives, can greatly impact the performance of Archeal on large archivals", "WARN".purple());
        let mut skip_list_elements: Vec<&str> = Vec::new();

        let file = std::fs::File::open(skip_list_path)?;
        let reader = BufReader::new(file);
        for line in reader.lines() {
            let line = line?;
        }

        let set1: HashSet<_> = unordered_anchor_links.into_iter().collect();
        let set2: HashSet<_> = skip_list_elements.into_iter().collect();
        let unique_elements: Vec<_> = set1.symmetric_difference(&set2).cloned().collect();

        for sublink in unique_elements {
            if sublink.starts_with("https://") || sublink.starts_with("http://") {
                anchor_links.push(sublink.to_string());
            } else {
                let host = parsed_url.host_str();
                let parts: Vec<&str> = host
                    .expect("Failed the transformation")
                    .split('.')
                    .collect();
                let root = if parts.len() >= 2 {
                    format!("{}.{}", parts[parts.len() - 2], parts[parts.len() - 1])
                } else {
                    host.expect("Failed the transformation").to_string()
                };
                let new_sublink = format!("https://{}/{}", &root, sublink);
                anchor_links.push(new_sublink);
            }
        }
    }
    Ok(anchor_links)
}
