use colored::*;
use std::{
    env,
    fs,
    io::prelude::*,
}; 

use scraper::{Html, Selector};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
       let url = args.get(1).cloned().unwrap_or_else(|| {
            println!("{}: No url has been provided", "Error".red());
            std::process::exit(1);
       });

       //let cloned_url = url.clone();
       let parsed_url = url::Url::parse(&url)?;
       let domain = parsed_url.domain().unwrap_or("unknown_domain");

       
       fs::create_dir_all(domain)?;
       
       println!("{}: Started to process: {}", "LOG".yellow().bold(), url.clone());

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

       let anchors_path = format!("{}/{}", domain, "anchors.txt");
       let html_path = format!("{}/{}", domain, "output.html");
       let anchors = fs::File::create(anchors_path);
       let html = fs::File::create(html_path);

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
