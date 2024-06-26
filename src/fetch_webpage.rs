use colored::*;
use reqwest::Client;

pub async fn fetch_page(url: String) -> anyhow::Result<String> {
    println!(
        "{}: Started to process: {}",
        "LOG".yellow().bold(),
        url.clone()
    );

    let client = Client::new();
    let resp = client.get(url.clone()).send().await?;

    let status = resp.status().to_string();
    let code: Option<&str> = status.split_whitespace().next();
    let result = match code {
        Some(code) => code.to_string(),
        None => String::from("Unknown"),
    };

    if !resp.status().is_success() {
        println!("{}: Response was not succesfull: {}", "Error".red(), result);
    }

    let html_body = resp.text().await?;
    Ok(html_body)
}
