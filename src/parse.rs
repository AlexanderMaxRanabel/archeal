use std::{fs, path::Path};

use chrono::prelude::*;
use colored::*;

pub async fn parse_and_create_dir(url: String) -> anyhow::Result<String> {
    let parsed_url = url::Url::parse(&url)?;

    let domain = parsed_url.domain().unwrap_or("unknown domain");

    let sanitized_domain: String = domain
        .chars()
        .filter(|&c| c.is_ascii_alphanumeric() || c == '-' || c == '.')
        .collect::<String>();

    let pathified_domain = Path::new(&sanitized_domain);
    if pathified_domain.exists() {
        println!(
            "{}: Path {} Already Exits, Recording into previously generated directory",
            "LOG".yellow().bold(),
            domain
        );
    } else {
        fs::create_dir_all(sanitized_domain.clone())?;
    }

    let utc: DateTime<Utc> = Utc::now();
    let formatted_utc = utc.format("%Y-%m-%dT%H%M%S%.f").to_string();
    let current_path: String = format!("{}/{}", sanitized_domain, formatted_utc);
    println!(
        "{}: To be written on: {}",
        "LOG".yellow().bold(),
        current_path.clone()
    );

    fs::create_dir(current_path.clone())?;
    Ok(current_path.clone())
}
