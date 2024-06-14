mod archive_page;
mod depth_fetch_webpage;
mod fetch_webpage;
mod order_weblinks;
mod parse;

use std::env;

use colored::*;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        let url = args.get(2).cloned().unwrap_or_else(|| {
            println!("{}: No url has been provided", "Error".red().bold());
            std::process::exit(1);
        });

        let depth = args.get(4).cloned().unwrap_or_else(|| {
            println!("{}: No depth has been provided", "Error".red().bold());
            std::process::exit(1);
        }); 

        let current_path = parse::parse_and_create_dir(url.clone()).await?;
        let mut html_body: String = fetch_webpage::fetch_page(url.clone()).await?;

        match depth.as_str() {
            "False" => {
                archive_page::archive_page(current_path.clone(), html_body.clone()).await?;
            }

            "True" => {
                html_body = depth_fetch_webpage::depth_fetch_page(
                    html_body.clone(),
                    current_path.clone(),
                    url.clone(),
                )
                .await?;

                archive_page::archive_page(current_path.clone(), html_body.clone()).await?;
            }

            _ => {
                println!("{}: Unknown depth mode: {}", "Error".red().bold(), depth);
            }
        }
    } else {
        println!("{}: Archive it", "Archeal".cyan().italic());
        println!(
            "{}: archeal --url https://example.com --depth False | True",
            "Usage".magenta()
        );
    }
    Ok(())
}
