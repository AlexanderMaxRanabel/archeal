use reqwest::Client;
use colored::*;
use tokio::{
    fs::File,
    io::AsyncWriteExt
};

fn get_after_last_period(input: &str) -> &str {
    match input.rfind('.') {
        Some(pos) => &input[pos + 1..],
        None => input,
    }
}


pub async fn fetch_webfile(url: String, current_path: String) -> anyhow::Result<()> {
    let client = Client::new();
    let mut response = client.get(url.clone()).send().await?;

    let after_period = get_after_last_period(url.as_str());

    if !response.status().is_success() {
        panic!("{}: Failed to download file: {}", "ERROR".red() ,response.status());
    }

    let file_path_string = format!("{}/main_file.{}", current_path, after_period);
    println!("{}", file_path_string.clone());

    let mut file = File::create(file_path_string.clone()).await?;

    while let Some(chunk) = response.chunk().await? {
        file.write_all(&chunk).await?;
    }

    println!("{}: File downloaded successfully on {}", "LOG".yellow(), file_path_string);

    Ok(())
}
