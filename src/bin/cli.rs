use anyhow::Result;
use clap::Parser;
use lector::scrape;

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    #[arg(short, long)]
    url: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    let scraped_data = scrape(&args.url).await?;

    println!("Title: {}", scraped_data.title);
    if let Some(author) = scraped_data.author {
        println!("Author: {}", author);
    }
    if let Some(site_name) = scraped_data.site_name {
        println!("Source: {}", site_name);
    }
    if let Some(published_time) = scraped_data.published_time {
        println!("Published: {}", published_time);
    }
    println!("Markdown Content:\n{}", scraped_data.markdown);

    Ok(())
}

