use colored::*;
use std::{
    env,
    fs,
    io::prelude::*,
    path::Path,
}; 

use scraper::{Html, Selector};
use chrono::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
       let url = args.get(1).cloned().unwrap_or_else(|| {
            println!("{}: No url has been provided", "Error".red());
            std::process::exit(1);
       });

       let parsed_url = url::Url::parse(&url)?;
       let domain = parsed_url.domain().unwrap_or("unknown domain");

       let sanitized_domain = domain.chars().filter(|&c| c.is_ascii_alphanumeric() || c == '-' || c == '.').collect::<String>();
       let pathified_domain = Path::new(&sanitized_domain);
       if pathified_domain.exists() {
           println!("{}: Path {} Already Exits", "WARN".yellow().italic(), domain);
       } else {
           fs::create_dir_all(sanitized_domain.clone())?;
       }

       let utc: DateTime<Utc> = Utc::now();
       let formatted_utc = utc.format("%Y-%m-%dT%H%M%S%.f").to_string();
       let current_path = format!("{}/{}", sanitized_domain, formatted_utc);
       fs::create_dir(current_path.clone())?;
       
       println!("{}: Started to process: {}", "LOG".yellow().bold(), url.clone());
       println!("{}", current_path);

       let resp = reqwest::get(url).await?;
       let status = resp.status().to_string();
       let code:Option<&str> = status.split_whitespace().next();
       let result = match code {
          Some(code) => code.to_string(),
          None => String::from("Unknown"),
       };
        
       if !resp.status().is_success() {
           println!("{}: Response was not succesfull: {}", "Error".red(), result);
           std::process::exit(1);
       }

       let html_body = resp.text().await?;

       let document = Html::parse_document(&html_body);

       let mut anchor_links: Vec<&str> = Vec::new();
       let anchor_selector = Selector::parse("a").unwrap();
       for link in document.select(&anchor_selector) {
            if let Some(href) = link.value().attr("href") {
                anchor_links.push(href);
            }
       }

       let anchors_path = format!("{}/{}", current_path, "anchors.txt");
       let html_path = format!("{}/{}", current_path, "output.html");
       let anchors = fs::File::create(anchors_path.clone());
       let html = fs::File::create(html_path.clone());

       println!("{}", anchors_path.clone());
       println!("{}", html_path.clone());

       let anchor_content: String = anchor_links.into_iter().map(|x| x.to_string()).collect();

       anchors.expect("Cannot write").write_all(anchor_content.as_bytes())?;
       html.expect("Cannot write").write_all(html_body.as_bytes())?;
    } else {
        println!("{}, Archive it", "Archeal".cyan().italic());
        println!("{}: No arguments was provided", "Error".red());
        std::process::exit(1);
    }
    Ok(())
}
