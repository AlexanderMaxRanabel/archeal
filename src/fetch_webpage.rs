use colored::*;

pub async fn fetch_page(url: String) -> anyhow::Result<String> {
    println!(
        "{}: Started to process: {}",
        "LOG".yellow().bold(),
        url.clone()
    );

    let resp = reqwest::get(url.clone()).await?;
    let status = resp.status().to_string();
    let code: Option<&str> = status.split_whitespace().next();
    let result = match code {
        Some(code) => code.to_string(),
        None => String::from("Unknown"),
    };

    if !resp.status().is_success() {
        println!("{}: Response was not succesfull: {}", "Error".red(), result);
        std::process::exit(1);
    }

    let html_body = resp.text().await?;
    Ok(html_body)
}
