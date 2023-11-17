use colored::*;
use std::{
    env,
    fs,
    io::{self, Write},
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

       let cloned_url = url.clone();
       let parsed_url = url::Url::parse(&cloned_url)?;
       let domain = parsed_url.domain().unwrap_or("unknown_domain");

       /*"let _ = */ fs::create_dir_all(domain)?;
       
       let resp = reqwest::get(url).await?;
       
       let html_body = resp.text().await?;

       let document = Html::parse_document(&html_body);

       let mut asset_links: Vec<&str> = Vec::new();
       let a_selector = Selector::parse("a").unwrap();
       for link in document.select(&a_selector) {
            if let Some(href) = link.value().attr("href") {
                asset_links.push(href);
            }
       }

       println!("{:?}", asset_links);
    } else {
        println!("{}, Archive it", "Archeal".cyan().italic());
        println!("{}: No arguments was provided", "Error".red());
        std::process::exit(1);
    }
    Ok(())
}
